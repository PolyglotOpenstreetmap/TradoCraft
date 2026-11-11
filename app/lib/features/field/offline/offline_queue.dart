import 'dart:async';
import 'dart:convert';

import 'package:dio/dio.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../app/api.dart';
import '../../../shared/photos.dart' show newClientId;
import 'kv_store.dart';

/// The technician writes that can wait for a connection (ADR 0022).
enum OpType {
  start,
  taskDone,
  taskAdd,
  workLog,
  partAdd,
  complete,
  decline,
  signature,
  photo,
}

/// One change made on the device and not yet accepted by the server.
///
/// [id] is the client UUID: it is sent to the server as the id (or `client_id`) of the record, so
/// replaying the operation any number of times creates the record once.
class QueuedOp {
  QueuedOp({
    required this.id,
    required this.type,
    required this.jobId,
    required this.jobLabel,
    required this.payload,
    required this.createdAt,
    this.entityId,
    this.attempts = 0,
    this.lastError,
    this.failed = false,
    this.nextAttemptAt,
    this.dataBytes = 0,
  });

  final String id;
  final OpType type;
  final String jobId;

  /// Shown in the list ("J2026-00001 Service pump station").
  final String jobLabel;
  final Map<String, Object?> payload;
  final DateTime createdAt;

  /// The record this operation changes, when it is not [id] itself (the task of a tick).
  final String? entityId;

  /// Size of the file kept next to the operation (photo, signature); 0 for none.
  final int dataBytes;
  int attempts;
  String? lastError;

  /// The server refused it for good: it stays in the list until retried or discarded, and blocks
  /// the later changes to the same job so that the order is kept.
  bool failed;
  DateTime? nextAttemptAt;

  /// Short text of what it does ("Replace seals").
  String get detail => switch (type) {
    OpType.taskDone || OpType.taskAdd => '${payload['description'] ?? ''}',
    OpType.workLog => '${payload['notes'] ?? ''}',
    OpType.partAdd => '${payload['description'] ?? ''}',
    OpType.complete => '${payload['notes'] ?? ''}',
    OpType.decline => '${payload['reason'] ?? ''}',
    OpType.signature => '${payload['signer_name'] ?? ''}',
    OpType.photo => '${payload['filename'] ?? ''}',
    OpType.start => '',
  };

  Map<String, Object?> toJson() => {
    'id': id,
    'type': type.name,
    'job_id': jobId,
    'job_label': jobLabel,
    'payload': payload,
    'created_at': createdAt.toUtc().toIso8601String(),
    'entity_id': entityId,
    'attempts': attempts,
    'last_error': lastError,
    'failed': failed,
    'next_attempt_at': nextAttemptAt?.toUtc().toIso8601String(),
    'data_bytes': dataBytes,
  };

  static QueuedOp? fromJson(Object? json) {
    if (json is! Map) return null;
    final type = OpType.values.where((t) => t.name == json['type']);
    if (type.isEmpty || json['id'] is! String || json['job_id'] is! String) {
      return null;
    }
    return QueuedOp(
      id: json['id'] as String,
      type: type.first,
      jobId: json['job_id'] as String,
      jobLabel: '${json['job_label'] ?? ''}',
      payload: Map<String, Object?>.from(json['payload'] as Map? ?? const {}),
      createdAt:
          DateTime.tryParse('${json['created_at']}') ?? DateTime.now().toUtc(),
      entityId: json['entity_id'] as String?,
      attempts: (json['attempts'] as num?)?.toInt() ?? 0,
      lastError: json['last_error'] as String?,
      failed: json['failed'] == true,
      nextAttemptAt: DateTime.tryParse('${json['next_attempt_at']}'),
      dataBytes: (json['data_bytes'] as num?)?.toInt() ?? 0,
    );
  }
}

/// The change could not be kept on the device.
class QueueFullException implements Exception {
  const QueueFullException(this.message);
  final String message;
  @override
  String toString() => message;
}

/// What the server said about one attempt.
enum SendResult { done, network, transient, permanent }

/// True when the request never got an answer: no signal, a timeout, a server that is down.
bool isNetworkError(Object e) => e is DioException && e.response == null;

/// A persistent, ordered list of changes made while offline, replayed when the server can be
/// reached again.
///
/// * First in, first out, per job: when a change of a job fails for good, the later changes of
///   that job wait behind it (other jobs carry on).
/// * A network error stops the run and tries again later with a growing delay; the periodic timer,
///   the app coming to the front and "Sync now" start a run.
/// * 2xx is done. 408, 429 and 5xx are tried again later. Any other 4xx (job reassigned or
///   cancelled, validation, no rights) is kept, with the server's message, until the user retries
///   or discards it. Nothing is dropped silently.
/// * The server accepts every operation twice (ADR 0006, 0022), so a run that was cut off after
///   the server saw the request is harmless when repeated.
class OfflineQueue extends ChangeNotifier {
  OfflineQueue({
    required this.dio,
    required this.store,
    DateTime Function()? now,
    this.maxBytesPerFile = 3 * 1024 * 1024,
    this.maxBytesTotal = 12 * 1024 * 1024,
  }) : _now = now ?? DateTime.now {
    ready = _load();
  }

  static const _indexKey = 'offline_queue_v1';
  static String _opKey(String id) => 'offline_queue_v1_op_$id';
  static String _dataKey(String id) => 'offline_queue_v1_data_$id';

  final Dio dio;
  final KvStore store;
  final DateTime Function() _now;
  final int maxBytesPerFile;
  final int maxBytesTotal;
  final _ops = <QueuedOp>[];
  final _applied = StreamController<AppliedOp>.broadcast();
  bool _running = false;
  bool _disposed = false;

  /// After a run that found no connection, the timer waits until this moment.
  DateTime? _pausedUntil;

  /// Completes when the operations saved on the device have been read.
  late final Future<void> ready;

  List<QueuedOp> get ops => List.unmodifiable(_ops);
  int get count => _ops.length;
  int get failedCount => _ops.where((o) => o.failed).length;
  bool get running => _running;
  List<QueuedOp> forJob(String jobId) =>
      _ops.where((o) => o.jobId == jobId).toList();
  bool hasJob(String jobId) => _ops.any((o) => o.jobId == jobId);

  /// Fires after an operation was accepted, with the job as the server now has it.
  Stream<AppliedOp> get applied => _applied.stream;

  Future<void> _load() async {
    try {
      final index = jsonDecode(await store.get(_indexKey) ?? '[]');
      for (final id in index is List ? index : const []) {
        final raw = await store.get(_opKey('$id'));
        final op = raw == null ? null : QueuedOp.fromJson(jsonDecode(raw));
        if (op != null && !_ops.any((o) => o.id == op.id)) _ops.add(op);
      }
    } catch (_) {
      // Unreadable storage: start empty rather than blocking the app.
    }
    if (!_disposed) notifyListeners();
  }

  Future<void> _saveIndex() =>
      store.set(_indexKey, jsonEncode([for (final o in _ops) o.id]));

  Future<void> _saveOp(QueuedOp op) =>
      store.set(_opKey(op.id), jsonEncode(op.toJson()));

  /// Adds a change at the end of the queue. Queueing an id that is already there changes nothing,
  /// so a dialog that is saved twice does not record the work twice.
  Future<QueuedOp> enqueue({
    required OpType type,
    required String jobId,
    required String jobLabel,
    required Map<String, Object?> payload,
    String? id,
    String? entityId,
    Uint8List? data,
  }) async {
    await ready;
    id ??= newClientId();
    final existing = _ops.where((o) => o.id == id);
    if (existing.isNotEmpty) return existing.first;
    if (data != null) {
      if (data.length > maxBytesPerFile) {
        throw QueueFullException(
          'File too large to keep offline (${data.length ~/ 1024} KB, '
          'at most ${maxBytesPerFile ~/ 1024} KB).',
        );
      }
      final used = _ops.fold<int>(0, (a, o) => a + o.dataBytes);
      if (used + data.length > maxBytesTotal) {
        throw const QueueFullException(
          'Too many files are waiting to sync on this device.',
        );
      }
    }
    final op = QueuedOp(
      id: id,
      type: type,
      jobId: jobId,
      jobLabel: jobLabel,
      payload: payload,
      createdAt: _now().toUtc(),
      entityId: entityId,
      dataBytes: data?.length ?? 0,
    );
    if (data != null) await store.set(_dataKey(id), base64Encode(data));
    await _saveOp(op);
    _ops.add(op);
    await _saveIndex();
    notifyListeners();
    return op;
  }

  /// Sends what is waiting, in order. Safe to call at any time; a run in progress is not doubled.
  /// [force] ignores the waiting time after earlier failures (the "Sync now" button).
  Future<void> process({bool force = false}) async {
    await ready;
    if (_running || _ops.isEmpty) return;
    final pause = _pausedUntil;
    if (!force && pause != null && pause.isAfter(_now())) return;
    _running = true;
    notifyListeners();
    try {
      final blocked = <String>{};
      for (final op in List.of(_ops)) {
        if (_disposed) return;
        if (blocked.contains(op.jobId)) continue;
        if (op.failed) {
          blocked.add(op.jobId);
          continue;
        }
        final due = op.nextAttemptAt;
        if (!force && due != null && due.isAfter(_now())) {
          blocked.add(op.jobId);
          continue;
        }
        Object? detail;
        String error = '';
        SendResult result;
        try {
          detail = await _send(op);
          result = SendResult.done;
        } catch (e) {
          error = errorMessage(e);
          result = _classify(e);
        }
        if (result == SendResult.done) {
          _pausedUntil = null;
          await _remove(op);
          if (!_applied.isClosed) {
            _applied.add(
              AppliedOp(
                op,
                detail is Map ? Map<String, Object?>.from(detail) : null,
              ),
            );
          }
          notifyListeners();
          continue;
        }
        op.attempts++;
        op.lastError = error;
        if (result == SendResult.permanent) {
          op.failed = true;
          op.nextAttemptAt = null;
          blocked.add(op.jobId);
        } else {
          op.nextAttemptAt = _now().add(backoff(op.attempts));
          if (result == SendResult.network) {
            _pausedUntil = op.nextAttemptAt;
            await _saveOp(op);
            notifyListeners();
            return; // no signal: the other jobs would fail the same way
          }
          blocked.add(op.jobId);
        }
        await _saveOp(op);
        notifyListeners();
      }
    } finally {
      _running = false;
      if (!_disposed) notifyListeners();
    }
  }

  /// How long to wait after the [attempts]th failure: 5 s, 10 s, 20 s ... at most 5 minutes.
  static Duration backoff(int attempts) {
    final seconds = 5 * (1 << (attempts - 1).clamp(0, 6));
    return Duration(seconds: seconds > 300 ? 300 : seconds);
  }

  static SendResult _classify(Object e) {
    if (e is! DioException) return SendResult.transient;
    final code = e.response?.statusCode;
    if (code == null) return SendResult.network;
    if (code == 408 || code == 429 || code >= 500) return SendResult.transient;
    if (code >= 400) return SendResult.permanent;
    return SendResult.transient;
  }

  /// Tries a failed change again (and the ones behind it).
  Future<void> retry(String id) async {
    await ready;
    final op = _ops.where((o) => o.id == id).firstOrNull;
    if (op == null) return;
    op.failed = false;
    op.nextAttemptAt = null;
    await _saveOp(op);
    notifyListeners();
    await process(force: true);
  }

  /// Throws a change away on the user's request.
  Future<void> discard(String id) async {
    await ready;
    final op = _ops.where((o) => o.id == id).firstOrNull;
    if (op == null) return;
    await _remove(op);
    notifyListeners();
  }

  Future<void> _remove(QueuedOp op) async {
    _ops.removeWhere((o) => o.id == op.id);
    await _saveIndex();
    await store.remove(_opKey(op.id));
    if (op.dataBytes > 0) await store.remove(_dataKey(op.id));
  }

  Future<Map<String, Object?>?> _send(QueuedOp op) async {
    final base = '/api/v1/field/jobs/${op.jobId}';
    final p = op.payload;
    Future<Response<Object?>> post(
      String path, {
      Object? data,
      Map<String, Object?>? query,
      Options? options,
    }) => dio.post<Object?>(
      path,
      data: data,
      queryParameters: query,
      options: options,
    );
    final Response<Object?> res;
    switch (op.type) {
      case OpType.start:
        res = await post('$base/start', data: <String, Object?>{});
      case OpType.taskDone:
        res = await post(
          '$base/tasks/${op.entityId}/done',
          data: {'done': p['done']},
        );
      case OpType.taskAdd:
        res = await post(
          '$base/tasks',
          data: {'description': p['description'], 'client_id': op.id},
        );
      case OpType.workLog:
        res = await post('$base/work-logs', data: {...p, 'id': op.id});
      case OpType.partAdd:
        res = await post('$base/parts', data: {...p, 'id': op.id});
      case OpType.complete:
        res = await post('$base/complete', data: {'notes': p['notes']});
      case OpType.decline:
        res = await post('$base/signature/declined', data: p);
      case OpType.signature:
        final bytes = await _bytes(op);
        res = await post(
          '$base/signature',
          data: bytes,
          query: {'signer_name': p['signer_name'], 'client_id': op.id},
          options: Options(
            contentType: 'image/png',
            headers: {Headers.contentLengthHeader: bytes.length},
          ),
        );
      case OpType.photo:
        final bytes = await _bytes(op);
        res = await post(
          '$base/photos',
          data: bytes,
          query: {
            'filename': p['filename'],
            'client_id': op.id,
            if ('${p['caption'] ?? ''}'.isNotEmpty) 'caption': p['caption'],
          },
          options: Options(
            contentType: 'application/octet-stream',
            headers: {Headers.contentLengthHeader: bytes.length},
          ),
        );
    }
    final data = res.data;
    return data is Map ? Map<String, Object?>.from(data) : null;
  }

  Future<Uint8List> _bytes(QueuedOp op) async {
    final raw = await store.get(_dataKey(op.id));
    if (raw == null) {
      throw DioException(
        requestOptions: RequestOptions(),
        response: Response(
          requestOptions: RequestOptions(),
          statusCode: 422,
          data: {'message': 'The file saved on this device is missing.'},
        ),
      );
    }
    return base64Decode(raw);
  }

  @override
  void dispose() {
    _disposed = true;
    _applied.close();
    super.dispose();
  }
}

/// An operation the server accepted, with the job as it answered (when it did).
class AppliedOp {
  const AppliedOp(this.op, this.detail);
  final QueuedOp op;
  final Map<String, Object?>? detail;
}

final offlineQueueProvider = Provider<OfflineQueue>((ref) {
  final queue = OfflineQueue(
    dio: ref.watch(dioProvider),
    store: ref.watch(userKvStoreProvider),
  );
  ref.onDispose(queue.dispose);
  return queue;
});

/// Sends a technician write now; if the server cannot be reached, or earlier changes of the job
/// are still waiting (the order must be kept), it is queued instead.
///
/// Returns the job as the server answered, or null when the change was queued. Errors from the
/// server (4xx/5xx) are thrown as before.
Future<JobDetail?> submitWrite(
  OfflineQueue queue, {
  required OpType type,
  required String jobId,
  required String jobLabel,
  required Future<JobDetail> Function() online,
  Map<String, Object?> payload = const {},
  String? id,
  String? entityId,
  Uint8List? data,
}) async {
  await queue.ready;
  Future<void> enqueue() => queue.enqueue(
    type: type,
    jobId: jobId,
    jobLabel: jobLabel,
    payload: payload,
    id: id,
    entityId: entityId,
    data: data,
  );
  if (queue.hasJob(jobId)) {
    await enqueue();
    unawaited(queue.process());
    return null;
  }
  try {
    return await online();
  } catch (e) {
    if (!isNetworkError(e)) rethrow;
    await enqueue();
    return null;
  }
}
