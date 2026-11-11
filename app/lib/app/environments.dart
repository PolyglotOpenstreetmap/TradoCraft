import 'dart:convert';

import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../features/field/offline/kv_store.dart';
import 'session.dart';
import 'workspace.dart';

/// What an environment is for (ADR 0043). The server decides; the app only shows it.
enum EnvironmentKind {
  live,
  test,
  tutorial;

  /// The kind written by the server (`live`, `test`, `tutorial`); null for anything else.
  static EnvironmentKind? parse(Object? raw) {
    for (final k in values) {
      if (k.name == raw) return k;
    }
    return null;
  }
}

/// One server the app can talk to: its address, what kind of data it holds and, per environment,
/// who is signed in there (the same fields as [Session]).
class AppEnvironment {
  const AppEnvironment({
    required this.id,
    required this.name,
    required this.kind,
    required this.serverUrl,
    this.devUser,
    this.accessToken,
    this.companyId,
  });

  final String id;
  final String name;
  final EnvironmentKind kind;
  final String serverUrl;
  final String? devUser;
  final String? accessToken;
  final String? companyId;

  /// The session of this environment (signed out when nobody signed in there yet).
  Session get session => Session(
    serverUrl: serverUrl,
    devUser: devUser,
    accessToken: accessToken,
    companyId: companyId,
  );

  /// A copy with the sign-in of [s] (its server address too).
  AppEnvironment withSession(Session s) => AppEnvironment(
    id: id,
    name: name,
    kind: kind,
    serverUrl: s.serverUrl,
    devUser: s.devUser,
    accessToken: s.accessToken,
    companyId: s.companyId,
  );

  /// A copy with another name and/or kind.
  AppEnvironment copyWith({String? name, EnvironmentKind? kind}) =>
      AppEnvironment(
        id: id,
        name: name ?? this.name,
        kind: kind ?? this.kind,
        serverUrl: serverUrl,
        devUser: devUser,
        accessToken: accessToken,
        companyId: companyId,
      );

  Map<String, Object?> toJson() => {
    'id': id,
    'name': name,
    'kind': kind.name,
    'url': serverUrl,
    'dev': devUser,
    'token': accessToken,
    'company': companyId,
  };

  static AppEnvironment? fromJson(Object? j) {
    if (j is! Map || j['id'] is! String || j['url'] is! String) return null;
    String? s(String k) => j[k] is String ? j[k] as String : null;
    return AppEnvironment(
      id: j['id'] as String,
      name: s('name') ?? hostOf(j['url'] as String),
      kind: EnvironmentKind.parse(j['kind']) ?? EnvironmentKind.live,
      serverUrl: j['url'] as String,
      devUser: s('dev'),
      accessToken: s('token'),
      companyId: s('company'),
    );
  }
}

/// The list of environments and which one is in use.
class EnvironmentsState {
  const EnvironmentsState({required this.items, required this.activeId});

  final List<AppEnvironment> items;
  final String activeId;

  AppEnvironment get active =>
      items.firstWhere((e) => e.id == activeId, orElse: () => items.first);

  EnvironmentsState copyWith({List<AppEnvironment>? items, String? activeId}) =>
      EnvironmentsState(
        items: items ?? this.items,
        activeId: activeId ?? this.activeId,
      );
}

/// The host (and port) of an address, used as the name of an environment nobody named.
String hostOf(String url) {
  final u = Uri.tryParse(url);
  if (u == null || u.host.isEmpty) return url;
  return u.hasPort ? '${u.host}:${u.port}' : u.host;
}

/// An address as stored: trimmed, without trailing slashes; null when it is not an http(s)
/// address with a host.
String? normalizeServerUrl(String raw) {
  var t = raw.trim();
  while (t.endsWith('/')) {
    t = t.substring(0, t.length - 1);
  }
  final u = Uri.tryParse(t);
  if (u == null || !(u.scheme == 'http' || u.scheme == 'https')) return null;
  return u.host.isEmpty ? null : t;
}

/// Reads and writes the list of environments in the device (or profile) storage.
class EnvironmentStore {
  static const key = 'environments';

  /// The list from [store]. Without one (an installation from before environments) the stored
  /// [session] becomes the first environment: kind `live` until the server says otherwise.
  static Future<EnvironmentsState> load(KvStore store, Session session) async {
    try {
      final raw = await store.get(key);
      if (raw != null) {
        final json = jsonDecode(raw) as Map<String, dynamic>;
        final items = <AppEnvironment>[
          for (final e in (json['items'] as List? ?? const []))
            ?AppEnvironment.fromJson(e),
        ];
        if (items.isNotEmpty) {
          final active = json['active'];
          return EnvironmentsState(
            items: items,
            activeId: items.any((e) => e.id == active)
                ? active as String
                : items.first.id,
          );
        }
      }
    } catch (_) {
      // Unreadable list: start again from the session below.
    }
    return fromSession(session);
  }

  /// A list with one environment made from [session].
  static EnvironmentsState fromSession(Session session) {
    final first = AppEnvironment(
      id: 'env-1',
      name: hostOf(session.serverUrl),
      kind: EnvironmentKind.live,
      serverUrl: session.serverUrl,
      devUser: session.devUser,
      accessToken: session.accessToken,
      companyId: session.companyId,
    );
    return EnvironmentsState(items: [first], activeId: first.id);
  }

  static Future<void> save(EnvironmentsState s, KvStore store) => store.set(
    key,
    jsonEncode({
      'active': s.activeId,
      'items': [for (final e in s.items) e.toJson()],
    }),
  );
}

/// What a server says about itself (`GET /api/v1/meta/environment`).
class EnvironmentProbe {
  const EnvironmentProbe({
    required this.name,
    required this.kind,
    required this.dbName,
    required this.version,
  });

  final String name;
  final EnvironmentKind kind;
  final String dbName;
  final String version;
}

/// Asks the server at an address which environment it is. Needs no sign-in. Overridden in tests.
final environmentProbeProvider =
    Provider<Future<EnvironmentProbe> Function(String serverUrl)>(
      (ref) => (url) async {
        final dio = Dio(
          BaseOptions(
            baseUrl: url,
            connectTimeout: const Duration(seconds: 8),
            receiveTimeout: const Duration(seconds: 8),
          ),
        );
        try {
          final res = await dio.get<Object?>('/api/v1/meta/environment');
          final d = res.data;
          final kind = d is Map ? EnvironmentKind.parse(d['kind']) : null;
          if (d is! Map || kind == null || d['name'] is! String) {
            throw const FormatException('Not a TradoCraft server');
          }
          return EnvironmentProbe(
            name: d['name'] as String,
            kind: kind,
            dbName: '${d['db_name'] ?? ''}',
            version: '${d['version'] ?? ''}',
          );
        } finally {
          dio.close();
        }
      },
    );

/// The environments known at start-up (overridden in `main` with the stored list).
final initialEnvironmentsProvider = Provider<EnvironmentsState>(
  (ref) => EnvironmentStore.fromSession(ref.read(initialSessionProvider)),
);

class EnvironmentsNotifier extends Notifier<EnvironmentsState> {
  @override
  EnvironmentsState build() {
    final initial = ref.read(initialEnvironmentsProvider);
    // The session is the truth for the active environment's sign-in: keep the list in step.
    ref.listen(sessionProvider, (_, s) => _followSession(s));
    Future.microtask(refreshActive);
    return initial;
  }

  void _followSession(Session s) {
    final a = state.active;
    final changedUrl = a.serverUrl != s.serverUrl;
    state = state.copyWith(
      items: [for (final e in state.items) e.id == a.id ? e.withSession(s) : e],
    );
    _persist();
    if (changedUrl) refreshActive();
  }

  Future<void> _persist() async {
    try {
      await EnvironmentStore.save(state, ref.read(kvStoreProvider));
    } catch (_) {
      // Not being able to remember the list must not break the session.
    }
  }

  /// Asks the active environment's server what it is and takes over its kind (and the name, when
  /// that is still the address). A server that does not answer leaves things as they are.
  Future<void> refreshActive() async {
    final a = state.active;
    try {
      final info = await ref.read(environmentProbeProvider)(a.serverUrl);
      if (!ref.mounted) return;
      final stillActive = state.active.id == a.id;
      if (!stillActive) return;
      final rename = a.name == hostOf(a.serverUrl);
      state = state.copyWith(
        items: [
          for (final e in state.items)
            e.id == a.id
                ? e.copyWith(name: rename ? info.name : null, kind: info.kind)
                : e,
        ],
      );
      await _persist();
    } catch (_) {
      // Unreachable or not a tradocraft server: keep the stored kind.
    }
  }

  /// Adds the environment at [serverUrl] (already checked with the server) and makes it active.
  /// Returns false when that address is already in the list.
  Future<bool> add({
    required String name,
    required String serverUrl,
    required EnvironmentKind kind,
  }) async {
    if (state.items.any((e) => e.serverUrl == serverUrl)) return false;
    var n = state.items.length + 1;
    while (state.items.any((e) => e.id == 'env-$n')) {
      n++;
    }
    final env = AppEnvironment(
      id: 'env-$n',
      name: name,
      kind: kind,
      serverUrl: serverUrl,
    );
    state = state.copyWith(items: [...state.items, env]);
    await _persist();
    await switchTo(env.id);
    return true;
  }

  /// Makes [id] the active environment: the open tabs are closed and the session is replaced by
  /// the one of that environment, so the HTTP client, the company and the storage of the user
  /// all change together and nothing of the previous environment stays on screen.
  Future<void> switchTo(String id) async {
    final target = state.items.where((e) => e.id == id).firstOrNull;
    if (target == null || target.id == state.activeId) return;
    ref.read(workspaceProvider.notifier).closeAll();
    state = state.copyWith(activeId: id);
    await _persist();
    await ref.read(sessionProvider.notifier).signIn(target.session);
    refreshActive();
  }

  /// Removes [id] from this device; the active environment cannot be removed.
  Future<void> remove(String id) async {
    if (id == state.activeId) return;
    state = state.copyWith(
      items: [
        for (final e in state.items)
          if (e.id != id) e,
      ],
    );
    await _persist();
  }
}

final environmentsProvider =
    NotifierProvider<EnvironmentsNotifier, EnvironmentsState>(
      EnvironmentsNotifier.new,
    );

/// The environment in use.
final activeEnvironmentProvider = Provider<AppEnvironment>(
  (ref) => ref.watch(environmentsProvider).active,
);
