import 'dart:convert';
import 'dart:typed_data';

import 'package:dio/dio.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:tradocraft_app/features/field/offline/kv_store.dart';
import 'package:tradocraft_app/features/field/offline/offline_queue.dart';

/// A server that answers from a script and records what it was sent.
class _Server implements HttpClientAdapter {
  /// Decides the answer for each request: a status code, or `null` for "no connection".
  int? Function(RequestOptions o) answer = (_) => 200;
  final sent = <String>[];
  final bodies = <Object?>[];
  final queries = <Map<String, dynamic>>[];

  @override
  Future<ResponseBody> fetch(
    RequestOptions o,
    Stream<Uint8List>? body,
    Future<void>? cancel,
  ) async {
    sent.add('${o.method} ${o.path}');
    queries.add(o.queryParameters);
    bodies.add(o.data);
    final status = answer(o);
    if (status == null) {
      throw DioException.connectionError(
        requestOptions: o,
        reason: 'no signal',
      );
    }
    return ResponseBody.fromString(
      jsonEncode(
        status < 300
            ? {'job': {}}
            : {'error': 'conflict', 'message': 'The job is cancelled'},
      ),
      status,
      headers: {
        Headers.contentTypeHeader: [Headers.jsonContentType],
      },
    );
  }

  @override
  void close({bool force = false}) {}
}

const _a = '/api/v1/field/jobs/a';
const _b = '/api/v1/field/jobs/b';

void main() {
  late _Server server;
  late MemoryKvStore store;
  var clock = DateTime.utc(2026, 11, 9, 8);

  OfflineQueue open() => OfflineQueue(
    dio: Dio(BaseOptions(baseUrl: 'http://test'))..httpClientAdapter = server,
    store: store,
    now: () => clock,
    maxBytesPerFile: 100,
    maxBytesTotal: 150,
  );

  setUp(() {
    server = _Server();
    store = MemoryKvStore();
    clock = DateTime.utc(2026, 11, 9, 8);
  });

  Future<void> add(
    OfflineQueue q,
    OpType type,
    String job, {
    String? id,
    String? entityId,
    Map<String, Object?> payload = const {},
    Uint8List? data,
  }) => q.enqueue(
    type: type,
    jobId: job,
    jobLabel: 'J-$job',
    id: id,
    entityId: entityId,
    payload: payload,
    data: data,
  );

  test(
    'changes are sent in the order they were made, then forgotten',
    () async {
      final q = open();
      await add(q, OpType.start, 'a');
      await add(
        q,
        OpType.taskDone,
        'a',
        entityId: 't1',
        payload: {'done': true},
      );
      await add(q, OpType.workLog, 'a', id: 'w1', payload: {'kind': 'work'});
      await add(q, OpType.complete, 'a', payload: {'notes': 'Done'});
      expect(q.count, 4);
      await q.process();
      expect(server.sent, [
        'POST $_a/start',
        'POST $_a/tasks/t1/done',
        'POST $_a/work-logs',
        'POST $_a/complete',
      ]);
      expect(q.count, 0);
      expect((server.bodies[2] as Map)['id'], 'w1', reason: 'the client id');
      expect(
        store.data.keys.where(
          (k) => k.contains('_op_') || k.contains('_data_'),
        ),
        isEmpty,
        reason: 'nothing is left on the device',
      );
      expect(store.data['offline_queue_v1'], '[]');
    },
  );

  test(
    'no connection: the change stays, with a waiting time that grows',
    () async {
      final q = open();
      await add(q, OpType.start, 'a');
      await add(q, OpType.start, 'b');
      server.answer = (_) => null;
      await q.process();
      expect(server.sent, hasLength(1), reason: 'a dead network stops the run');
      expect(q.count, 2);
      final first = q.ops.first;
      expect(first.attempts, 1);
      expect(first.failed, isFalse);
      expect(first.nextAttemptAt, clock.add(const Duration(seconds: 5)));

      // Too early: it is not tried again by the timer ...
      await q.process();
      expect(server.sent, hasLength(1));
      // ... but "Sync now" ignores the waiting time.
      server.answer = (_) => 200;
      await q.process(force: true);
      expect(q.count, 0);
      expect(server.sent, hasLength(3));

      expect(OfflineQueue.backoff(1), const Duration(seconds: 5));
      expect(OfflineQueue.backoff(3), const Duration(seconds: 20));
      expect(OfflineQueue.backoff(30), const Duration(minutes: 5));
    },
  );

  test('after the waiting time the timer retries by itself', () async {
    final q = open();
    await add(q, OpType.start, 'a');
    server.answer = (_) => null;
    await q.process();
    server.answer = (_) => 200;
    clock = clock.add(const Duration(seconds: 6));
    await q.process();
    expect(q.count, 0);
  });

  test('server errors 5xx, 408 and 429 are retried, not given up', () async {
    final q = open();
    await add(q, OpType.start, 'a');
    await add(q, OpType.start, 'b');
    for (final code in [503, 408, 429]) {
      server.answer = (_) => code;
      await q.process(force: true);
      expect(q.count, 2, reason: '$code');
      expect(q.ops.every((o) => !o.failed), isTrue, reason: '$code');
    }
    expect(q.ops.first.attempts, 3);
  });

  test(
    'a refusal is kept with the server message and blocks only its job',
    () async {
      final q = open();
      await add(q, OpType.start, 'a', id: 'a1');
      await add(q, OpType.complete, 'a', id: 'a2', payload: {'notes': 'x'});
      await add(q, OpType.start, 'b', id: 'b1');
      server.answer = (o) => o.path.startsWith(_a) ? 409 : 200;
      await q.process();
      // The first change of job a is refused; its second waits behind it; job b still goes.
      expect(server.sent, ['POST $_a/start', 'POST $_b/start']);
      expect(q.ops.map((o) => o.id), ['a1', 'a2']);
      expect(q.ops.first.failed, isTrue);
      expect(q.ops.first.lastError, 'The job is cancelled');
      expect(q.failedCount, 1);

      // Running again does not resend it and does not drop it.
      await q.process(force: true);
      expect(server.sent, hasLength(2));
      expect(q.count, 2);

      // Retry after the server agrees: it goes, and so does the one behind it.
      server.answer = (_) => 200;
      await q.retry('a1');
      expect(q.count, 0);
      expect(server.sent.skip(2), ['POST $_a/start', 'POST $_a/complete']);
    },
  );

  test(
    'discarding removes a change, and what was behind it moves on',
    () async {
      final q = open();
      await add(q, OpType.start, 'a', id: 'a1');
      await add(q, OpType.complete, 'a', id: 'a2', payload: {'notes': 'x'});
      server.answer = (_) => 404;
      await q.process();
      expect(q.ops.first.failed, isTrue);
      await q.discard('a1');
      expect(q.ops.map((o) => o.id), ['a2']);
      server.answer = (_) => 200;
      await q.process();
      expect(q.count, 0);
      expect(server.sent.last, 'POST $_a/complete');
    },
  );

  test(
    'the queue survives a restart; the same change is never queued twice',
    () async {
      var q = open();
      await add(
        q,
        OpType.taskAdd,
        'a',
        id: 'x1',
        payload: {'description': 'Check'},
      );
      await add(
        q,
        OpType.taskAdd,
        'a',
        id: 'x1',
        payload: {'description': 'Check'},
      );
      await add(q, OpType.workLog, 'a', id: 'x2', payload: {'kind': 'work'});
      expect(q.count, 2, reason: 'saving a dialog twice queues it once');
      server.answer = (_) => null;
      await q.process();
      expect(q.ops.first.attempts, 1);
      q.dispose();

      q = open(); // the app was closed and opened again
      await q.ready;
      expect(q.ops.map((o) => o.id), ['x1', 'x2']);
      expect(q.ops.first.attempts, 1, reason: 'the history is kept');
      expect(q.ops.first.payload['description'], 'Check');
      expect(q.ops.first.lastError, isNotEmpty);

      server.answer = (_) => 200;
      await q.process(force: true);
      expect(q.count, 0);
      // The replay carries the client id, which is what makes it safe on the server.
      expect((server.bodies.last as Map)['id'], 'x2');
      expect(server.bodies[server.bodies.length - 2], {
        'description': 'Check',
        'client_id': 'x1',
      });

      q.dispose();
      q = open();
      await q.ready;
      expect(q.count, 0, reason: 'once sent it is gone for good');
      q.dispose();
    },
  );

  test('a photo is kept as a file next to the change and sent as is', () async {
    var q = open();
    final bytes = Uint8List.fromList(List.generate(60, (i) => i));
    await add(
      q,
      OpType.photo,
      'a',
      id: 'p1',
      payload: {'filename': 'IMG.jpg', 'caption': ''},
      data: bytes,
    );
    server.answer = (_) => null;
    await q.process();
    q.dispose();

    q = open();
    server.answer = (_) => 200;
    await q.process(force: true);
    expect(server.sent.last, 'POST $_a/photos');
    expect(server.bodies.last, bytes);
    expect(server.queries.last['client_id'], 'p1');
    expect(server.queries.last['filename'], 'IMG.jpg');
    expect(store.data.keys, ['offline_queue_v1']);
    expect(store.data['offline_queue_v1'], '[]');
    q.dispose();
  });

  test(
    'files that are too big for the device queue are refused, not lost',
    () async {
      final q = open();
      await expectLater(
        add(q, OpType.photo, 'a', data: Uint8List(101)),
        throwsA(isA<QueueFullException>()),
      );
      await add(q, OpType.photo, 'a', data: Uint8List(100));
      await expectLater(
        add(q, OpType.photo, 'a', data: Uint8List(60)),
        throwsA(isA<QueueFullException>()),
        reason: 'the total is limited as well',
      );
      expect(q.count, 1);
      q.dispose();
    },
  );

  test(
    'submitWrite sends at once, queues on no signal, and keeps the order',
    () async {
      final q = open();
      Future<Never> live() async => throw StateError('not used');
      expect(live, isA<Function>());
      var calls = 0;
      Future<Never> offline() async {
        calls++;
        throw DioException.connectionError(
          requestOptions: RequestOptions(),
          reason: 'x',
        );
      }

      final r = await submitWrite(
        q,
        type: OpType.start,
        jobId: 'a',
        jobLabel: 'J',
        online: offline,
      );
      expect(r, isNull);
      expect(q.count, 1);
      // A second change of the same job is queued without trying the network (order!).
      server.answer = (_) => null;
      await submitWrite(
        q,
        type: OpType.complete,
        jobId: 'a',
        jobLabel: 'J',
        payload: const {'notes': 'x'},
        online: offline,
      );
      expect(calls, 1);
      expect(q.count, 2);
      // A refusal from the server is not queued: it is reported.
      await expectLater(
        submitWrite(
          q,
          type: OpType.start,
          jobId: 'z',
          jobLabel: 'J',
          online: () async => throw DioException(
            requestOptions: RequestOptions(),
            response: Response(
              requestOptions: RequestOptions(),
              statusCode: 409,
            ),
          ),
        ),
        throwsA(isA<DioException>()),
      );
      expect(q.count, 2);
      q.dispose();
    },
  );
}
