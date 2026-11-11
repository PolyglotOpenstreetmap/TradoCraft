import 'dart:convert';

import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../app/api.dart';
import 'kv_store.dart';

/// A copy of what the technician last saw: the list of their jobs and the jobs they opened, so
/// they can be read without a connection. Read-only; changes go through the offline queue.
class JobCache {
  JobCache(this._store, {DateTime Function()? now})
    : _now = now ?? DateTime.now;

  final KvStore _store;
  final DateTime Function() _now;

  static const _listKey = 'field_jobs_cache_v1';
  static const _indexKey = 'field_job_cache_v1_index';
  static String _key(String id) => 'field_job_cache_v1_$id';

  /// Keep the most recent jobs only: it is a convenience, not a database.
  static const maxDetails = 30;

  Future<void> saveList(List<Job> jobs) => _store.set(
    _listKey,
    jsonEncode({'saved_at': _now().toUtc().toIso8601String(), 'items': jobs}),
  );

  Future<({List<Job> jobs, DateTime savedAt})?> loadList() async {
    final m = await _read(_listKey);
    if (m == null) return null;
    try {
      return (
        jobs: [
          for (final j in m['items'] as List)
            Job.fromJson(Map<String, dynamic>.from(j as Map)),
        ],
        savedAt: DateTime.parse('${m['saved_at']}'),
      );
    } catch (_) {
      return null;
    }
  }

  Future<void> saveDetail(JobDetail d) async {
    final id = d.job.id;
    await _store.set(
      _key(id),
      jsonEncode({'saved_at': _now().toUtc().toIso8601String(), 'item': d}),
    );
    final index = await _index()
      ..remove(id)
      ..insert(0, id);
    while (index.length > maxDetails) {
      await _store.remove(_key(index.removeLast()));
    }
    await _store.set(_indexKey, jsonEncode(index));
  }

  Future<({JobDetail detail, DateTime savedAt})?> loadDetail(String id) async {
    final m = await _read(_key(id));
    if (m == null) return null;
    try {
      return (
        detail: JobDetail.fromJson(Map<String, dynamic>.from(m['item'] as Map)),
        savedAt: DateTime.parse('${m['saved_at']}'),
      );
    } catch (_) {
      return null;
    }
  }

  Future<List<String>> _index() async {
    try {
      final raw = await _store.get(_indexKey);
      return [...(jsonDecode(raw ?? '[]') as List).map((e) => '$e')];
    } catch (_) {
      return [];
    }
  }

  Future<Map<String, dynamic>?> _read(String key) async {
    try {
      final raw = await _store.get(key);
      if (raw == null) return null;
      return Map<String, dynamic>.from(jsonDecode(raw) as Map);
    } catch (_) {
      return null;
    }
  }
}

final jobCacheProvider = Provider<JobCache>(
  (ref) => JobCache(ref.watch(userKvStoreProvider)),
);
