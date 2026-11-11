import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:flutter/foundation.dart' show kIsWeb;
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:path/path.dart' as p;
import 'package:path_provider/path_provider.dart';

import '../features/field/offline/kv_store.dart';

/// A profile is a named, isolated session: its own sign-in, language, offline queue and open
/// tabs, kept in a file of its own. That lets one machine run several windows at once, for
/// instance an admin and a sales user, without them overwriting each other's settings
/// (`SharedPreferences` is one file per app and not safe to share between processes).
///
/// Chosen with `--profile=<name>` or the `TRAYD_PROFILE` environment variable; without either the
/// app behaves as before and uses `SharedPreferences`.
String? parseProfile(List<String> args, [Map<String, String>? env]) {
  String? raw;
  for (var i = 0; i < args.length; i++) {
    final a = args[i];
    if (a.startsWith('--profile=')) {
      raw = a.substring('--profile='.length);
    } else if (a == '--profile' && i + 1 < args.length) {
      raw = args[i + 1];
    }
  }
  // `Platform.environment` is not available on the web: there only a supplied lookup counts.
  final lookup = env ?? (kIsWeb ? null : Platform.environment);
  raw ??= lookup?['TRAYD_PROFILE'];
  return sanitizeProfile(raw);
}

/// Keeps a profile name usable as a file name; null when nothing usable is left.
String? sanitizeProfile(String? raw) {
  final cleaned = (raw ?? '')
      .trim()
      .replaceAll(RegExp(r'[^A-Za-z0-9_.-]'), '_')
      .replaceAll(RegExp(r'^[._-]+'), '');
  if (cleaned.isEmpty) return null;
  return cleaned.length > 32 ? cleaned.substring(0, 32) : cleaned;
}

/// The active profile name (null = the default, unnamed profile); overridden in `main`.
final profileProvider = Provider<String?>((ref) => null);

/// Key/value storage in one JSON file. Writes go to a temporary file that is then renamed, so a
/// crash cannot leave a half-written file behind.
class JsonFileKvStore implements KvStore {
  /// A store that keeps its data in [file].
  JsonFileKvStore(this.file);

  /// The JSON file holding all keys.
  final File file;
  Future<Map<String, String>>? _data;
  Future<void> _writes = Future.value();

  /// The store of [profile] in the application support directory (or [dir], for tests).
  static JsonFileKvStore forProfile(String profile, {required Directory dir}) =>
      JsonFileKvStore(File(p.join(dir.path, 'profiles', '$profile.json')));

  /// The store of [profile] in the platform's application support directory.
  static Future<JsonFileKvStore> openProfile(String profile) async =>
      forProfile(profile, dir: await getApplicationSupportDirectory());

  // One load for all callers, so concurrent first calls share the same map.
  Future<Map<String, String>> _load() => _data ??= _read();

  Future<Map<String, String>> _read() async {
    var data = <String, String>{};
    try {
      if (await file.exists()) {
        final json = jsonDecode(await file.readAsString());
        if (json is Map) {
          data = {
            for (final e in json.entries)
              if (e.value is String) e.key as String: e.value as String,
          };
        }
      }
    } catch (_) {
      // An unreadable file is treated as empty: the user signs in again.
    }
    return data;
  }

  /// Writes [data] to disk, one write after the other.
  Future<void> _flush(Map<String, String> data) {
    // Serialised: concurrent set() calls must not interleave their renames.
    return _writes = _writes.catchError((Object _) {}).then((_) async {
      await file.parent.create(recursive: true);
      final tmp = File('${file.path}.tmp');
      await tmp.writeAsString(jsonEncode(data), flush: true);
      await tmp.rename(file.path);
    });
  }

  @override
  Future<String?> get(String key) async => (await _load())[key];

  @override
  Future<void> set(String key, String value) async {
    final d = await _load();
    d[key] = value;
    await _flush(d);
  }

  @override
  Future<void> remove(String key) async {
    final d = await _load();
    if (d.remove(key) != null) await _flush(d);
  }
}
