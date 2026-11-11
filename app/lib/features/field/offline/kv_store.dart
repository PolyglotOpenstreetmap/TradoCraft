import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:shared_preferences/shared_preferences.dart';

import '../../../app/session.dart';

/// The small slice of key/value storage the offline queue and the job cache need.
abstract class KvStore {
  Future<String?> get(String key);
  Future<void> set(String key, String value);
  Future<void> remove(String key);
}

/// Keeps everything in memory (tests, and the fallback when the platform store is not available).
class MemoryKvStore implements KvStore {
  final Map<String, String> data = {};

  @override
  Future<String?> get(String key) async => data[key];

  @override
  Future<void> set(String key, String value) async => data[key] = value;

  @override
  Future<void> remove(String key) async => data.remove(key);
}

/// [SharedPreferences], opened on first use. When it cannot be opened (a platform without the
/// plugin, a widget test) the data lives in memory for the rest of the session instead.
class PrefsKvStore implements KvStore {
  Future<KvStore>? _inner;

  Future<KvStore> _open() => _inner ??= SharedPreferences.getInstance()
      .then<KvStore>(_Prefs.new)
      .catchError((Object _) => MemoryKvStore());

  @override
  Future<String?> get(String key) async => (await _open()).get(key);

  @override
  Future<void> set(String key, String value) async =>
      (await _open()).set(key, value);

  @override
  Future<void> remove(String key) async => (await _open()).remove(key);
}

class _Prefs implements KvStore {
  _Prefs(this._p);
  final SharedPreferences _p;

  @override
  Future<String?> get(String key) async => _p.getString(key);

  @override
  Future<void> set(String key, String value) async {
    if (!await _p.setString(key, value)) {
      throw StateError('Could not save $key on this device');
    }
  }

  @override
  Future<void> remove(String key) async {
    await _p.remove(key);
  }
}

/// Prefixes every key, so that two people using the same device do not see (or send) each
/// other's waiting changes and cached jobs.
class ScopedKvStore implements KvStore {
  ScopedKvStore(this._inner, this._scope);
  final KvStore _inner;
  final String _scope;

  String _k(String key) => '$_scope:$key';

  @override
  Future<String?> get(String key) => _inner.get(_k(key));

  @override
  Future<void> set(String key, String value) => _inner.set(_k(key), value);

  @override
  Future<void> remove(String key) => _inner.remove(_k(key));
}

/// The device storage; tests override it with a [MemoryKvStore].
final kvStoreProvider = Provider<KvStore>((ref) => PrefsKvStore());

/// The storage of the signed-in user: the dev user's e-mail, or the company when signed in with a
/// token (the token itself changes and is never used as a key).
final userKvStoreProvider = Provider<KvStore>((ref) {
  final s = ref.watch(sessionProvider);
  final who = (s.devUser?.isNotEmpty ?? false)
      ? 'dev:${s.devUser}'
      : 'token:${s.companyId ?? ''}';
  return ScopedKvStore(ref.watch(kvStoreProvider), '${s.serverUrl}|$who');
});
