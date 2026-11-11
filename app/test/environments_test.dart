import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:tradocraft_app/app/api.dart';
import 'package:tradocraft_app/app/environment_ui.dart';
import 'package:tradocraft_app/app/environments.dart';
import 'package:tradocraft_app/app/session.dart';
import 'package:tradocraft_app/app/shell.dart';
import 'package:tradocraft_app/app/workspace.dart';
import 'package:tradocraft_app/features/field/offline/kv_store.dart';
import 'package:tradocraft_app/features/login/login_page.dart';
import 'package:tradocraft_app/l10n/app_localizations.dart';

import 'fake_http.dart';

const _liveUrl = 'https://erp.example.com';
const _testUrl = 'http://localhost:8081';

/// What each fake server says about itself, by address.
final _servers = <String, EnvironmentProbe>{
  _liveUrl: const EnvironmentProbe(
    name: 'Acme production',
    kind: EnvironmentKind.live,
    dbName: 'acme',
    version: '0.1.0',
  ),
  _testUrl: const EnvironmentProbe(
    name: 'Acme test',
    kind: EnvironmentKind.test,
    dbName: 'acme_test',
    version: '0.1.0',
  ),
};

Future<EnvironmentProbe> _probe(String url) async =>
    _servers[url] ?? (throw Exception('no server at $url'));

AppEnvironment _env(
  String id,
  String name,
  EnvironmentKind kind,
  String url, {
  String? dev,
  String? company,
}) => AppEnvironment(
  id: id,
  name: name,
  kind: kind,
  serverUrl: url,
  devUser: dev,
  companyId: company,
);

EnvironmentsState _two() => EnvironmentsState(
  items: [
    _env(
      'env-1',
      'Acme production',
      EnvironmentKind.live,
      _liveUrl,
      dev: 'admin@example.com',
      company: 'company-live',
    ),
    _env(
      'env-2',
      'Acme test',
      EnvironmentKind.test,
      _testUrl,
      dev: 'sales@example.com',
      company: 'company-test',
    ),
  ],
  activeId: 'env-1',
);

ProviderContainer _container({
  EnvironmentsState? environments,
  MemoryKvStore? kv,
  Future<EnvironmentProbe> Function(String)? probe,
}) {
  final envs = environments ?? _two();
  final c = ProviderContainer(
    overrides: [
      kvStoreProvider.overrideWithValue(kv ?? MemoryKvStore()),
      initialEnvironmentsProvider.overrideWithValue(envs),
      initialSessionProvider.overrideWithValue(envs.active.session),
      environmentProbeProvider.overrideWithValue(probe ?? _probe),
    ],
  );
  addTearDown(c.dispose);
  return c;
}

Widget _app(ProviderContainer c, Widget home) => UncontrolledProviderScope(
  container: c,
  child: MaterialApp(
    locale: const Locale('en'),
    localizationsDelegates: AppLocalizations.localizationsDelegates,
    supportedLocales: AppLocalizations.supportedLocales,
    home: home,
  ),
);

void main() {
  group('storage and migration', () {
    test('an existing single session becomes the first environment', () async {
      final kv = MemoryKvStore();
      const session = Session(
        serverUrl: _liveUrl,
        devUser: 'admin@example.com',
        companyId: 'co-1',
      );
      await SessionStore.save(session, kv);

      final loaded = await SessionStore.load(kv);
      final state = await EnvironmentStore.load(kv, loaded);

      expect(state.items, hasLength(1));
      final e = state.active;
      expect(e.kind, EnvironmentKind.live);
      expect(e.name, 'erp.example.com');
      expect(e.serverUrl, _liveUrl);
      expect(e.session.devUser, 'admin@example.com');
      expect(e.session.companyId, 'co-1');
      // The old keys are untouched, so nothing is lost when going back to an older build.
      expect(await kv.get('dev_user'), 'admin@example.com');
    });

    test('the list survives a restart, with the sign-in of each', () async {
      final kv = MemoryKvStore();
      await EnvironmentStore.save(_two(), kv);

      final state = await EnvironmentStore.load(kv, Session.signedOut);

      expect(state.items.map((e) => e.name), ['Acme production', 'Acme test']);
      expect(state.items[1].kind, EnvironmentKind.test);
      expect(state.items[1].session.companyId, 'company-test');
      expect(state.items[0].session.companyId, 'company-live');
      expect(state.activeId, 'env-1');
    });

    test('an unreadable list falls back to the session', () async {
      final kv = MemoryKvStore()..data[EnvironmentStore.key] = '{nope';
      final state = await EnvironmentStore.load(
        kv,
        const Session(serverUrl: _liveUrl),
      );
      expect(state.items, hasLength(1));
      expect(state.active.serverUrl, _liveUrl);
    });

    test('an unknown kind in the stored list counts as live', () async {
      final kv = MemoryKvStore()
        ..data[EnvironmentStore.key] = jsonEncode({
          'active': 'a',
          'items': [
            {'id': 'a', 'name': 'X', 'kind': 'staging', 'url': _liveUrl},
          ],
        });
      final state = await EnvironmentStore.load(kv, Session.signedOut);
      expect(state.active.kind, EnvironmentKind.live);
    });

    test('server addresses are normalised', () {
      expect(
        normalizeServerUrl(' https://a.example.com// '),
        'https://a.example.com',
      );
      expect(normalizeServerUrl('a.example.com'), isNull);
      expect(normalizeServerUrl('ftp://a.example.com'), isNull);
      expect(normalizeServerUrl(''), isNull);
    });
  });

  group('the notifier', () {
    test(
      'the kind of the migrated environment is read from the server',
      () async {
        final c = _container(
          environments: EnvironmentStore.fromSession(
            const Session(serverUrl: _testUrl),
          ),
        );
        expect(c.read(environmentsProvider).active.kind, EnvironmentKind.live);
        c.read(environmentsProvider);
        await Future<void>.delayed(Duration.zero);
        await Future<void>.delayed(Duration.zero);
        final e = c.read(environmentsProvider).active;
        expect(e.kind, EnvironmentKind.test);
        // A name nobody chose (the address) is replaced by the server's.
        expect(e.name, 'Acme test');
      },
    );

    test('an unreachable server leaves the stored kind alone', () async {
      final c = _container(
        environments: EnvironmentStore.fromSession(
          const Session(serverUrl: 'http://nowhere.invalid'),
        ),
      );
      c.read(environmentsProvider);
      await Future<void>.delayed(Duration.zero);
      expect(c.read(environmentsProvider).active.kind, EnvironmentKind.live);
    });

    test('switching rebuilds the client for the new server and signs nobody else in', () async {
      final kv = MemoryKvStore();
      final c = _container(kv: kv);
      c.read(environmentsProvider);
      expect(c.read(dioProvider).options.baseUrl, _liveUrl);
      expect(
        c.read(dioProvider).options.headers['X-Company-Id'],
        'company-live',
      );

      await c.read(environmentsProvider.notifier).switchTo('env-2');

      final dio = c.read(dioProvider);
      expect(dio.options.baseUrl, _testUrl);
      expect(dio.options.headers['X-Dev-User'], 'sales@example.com');
      expect(dio.options.headers['X-Company-Id'], 'company-test');
      expect(c.read(sessionProvider).serverUrl, _testUrl);
      expect(c.read(environmentsProvider).activeId, 'env-2');
      // The choice is stored, with the session the next start reads.
      expect(await kv.get('server_url'), _testUrl);
      final stored = await EnvironmentStore.load(kv, Session.signedOut);
      expect(stored.activeId, 'env-2');
    });

    test('a new environment starts signed out, without a company', () async {
      final c = _container();
      c.read(environmentsProvider);
      final notifier = c.read(environmentsProvider.notifier);
      await notifier.add(
        name: 'Tutorial',
        serverUrl: 'http://localhost:8082',
        kind: EnvironmentKind.tutorial,
      );
      final s = c.read(sessionProvider);
      expect(s.serverUrl, 'http://localhost:8082');
      expect(s.isSignedIn, isFalse);
      expect(s.companyId, isNull);
      expect(
        c.read(dioProvider).options.headers.containsKey('X-Company-Id'),
        isFalse,
      );
      expect(c.read(activeEnvironmentProvider).kind, EnvironmentKind.tutorial);
    });

    test('signing in updates only the active environment', () async {
      final c = _container();
      c.read(environmentsProvider);
      await c
          .read(sessionProvider.notifier)
          .signIn(
            const Session(
              serverUrl: _liveUrl,
              devUser: 'manager@example.com',
              companyId: 'company-live-2',
            ),
          );
      final items = c.read(environmentsProvider).items;
      expect(items[0].devUser, 'manager@example.com');
      expect(items[0].companyId, 'company-live-2');
      expect(items[1].devUser, 'sales@example.com');
      expect(items[1].companyId, 'company-test');
    });

    test('opening tabs then switching clears the tabs', () async {
      final kv = MemoryKvStore();
      final c = _container(kv: kv);
      c.read(environmentsProvider);
      final ws = c.read(workspaceProvider.notifier);
      ws.open('/orders/o1');
      ws.open('/quotations/q1');
      expect(c.read(workspaceProvider).tabs, hasLength(2));

      await c.read(environmentsProvider.notifier).switchTo('env-2');
      await Future<void>.delayed(Duration.zero);

      expect(c.read(workspaceProvider).tabs, isEmpty);
      // Going back does not bring the old tabs back either.
      await c.read(environmentsProvider.notifier).switchTo('env-1');
      await Future<void>.delayed(Duration.zero);
      expect(c.read(workspaceProvider).tabs, isEmpty);
    });

    test('the active environment cannot be removed, another can', () async {
      final c = _container();
      c.read(environmentsProvider);
      final n = c.read(environmentsProvider.notifier);
      await n.remove('env-1');
      expect(c.read(environmentsProvider).items, hasLength(2));
      await n.remove('env-2');
      expect(c.read(environmentsProvider).items.map((e) => e.id), ['env-1']);
    });

    test('the same server cannot be added twice', () async {
      final c = _container();
      c.read(environmentsProvider);
      final ok = await c
          .read(environmentsProvider.notifier)
          .add(name: 'Again', serverUrl: _testUrl, kind: EnvironmentKind.test);
      expect(ok, isFalse);
      expect(c.read(environmentsProvider).items, hasLength(2));
    });
  });

  group('banner', () {
    for (final kind in EnvironmentKind.values) {
      for (final brightness in Brightness.values) {
        testWidgets('${kind.name} in ${brightness.name} mode', (tester) async {
          final envs = EnvironmentsState(
            items: [_env('e', 'Acme', kind, 'http://unknown.invalid')],
            activeId: 'e',
          );
          final c = _container(environments: envs);
          await tester.pumpWidget(
            UncontrolledProviderScope(
              container: c,
              child: MaterialApp(
                theme: ThemeData(brightness: brightness),
                locale: const Locale('en'),
                localizationsDelegates: AppLocalizations.localizationsDelegates,
                supportedLocales: AppLocalizations.supportedLocales,
                home: const Scaffold(body: EnvironmentBanner()),
              ),
            ),
          );
          await tester.pump();
          final word = {
            EnvironmentKind.live: 'LIVE',
            EnvironmentKind.test: 'TEST',
            EnvironmentKind.tutorial: 'TUTORIAL',
          }[kind]!;
          // The kind is written out, whatever the colour.
          expect(find.text('$word · Acme'), findsOneWidget);
          final material = tester.widget<Material>(
            find.byKey(ValueKey('env-banner-${kind.name}')),
          );
          final colors = environmentColors(kind, brightness);
          expect(material.color, colors.background);
          expect(
            tester.getSemantics(find.byType(EnvironmentBanner)).label,
            contains('Acme'),
          );
          expect(
            tester.getSemantics(find.byType(EnvironmentBanner)).label,
            contains(word[0] + word.substring(1).toLowerCase()),
          );
        });
      }
    }

    test('colours: red live, amber test, blue tutorial', () {
      Color bg(EnvironmentKind k) =>
          environmentColors(k, Brightness.light).background;
      expect(HSLColor.fromColor(bg(EnvironmentKind.live)).hue, lessThan(10));
      expect(
        HSLColor.fromColor(bg(EnvironmentKind.test)).hue,
        inInclusiveRange(35, 50),
      );
      expect(
        HSLColor.fromColor(bg(EnvironmentKind.tutorial)).hue,
        inInclusiveRange(200, 225),
      );
    });

    for (final width in [1200.0, 500.0]) {
      testWidgets('the shell shows it at width $width', (tester) async {
        tester.view.physicalSize = Size(width, 900);
        tester.view.devicePixelRatio = 1;
        addTearDown(tester.view.reset);
        final http = FakeHttp({
          'GET /api/v1/me': (_) => meJson(['orders.read', 'helpdesk.read']),
        });
        final c = ProviderContainer(
          overrides: [
            dioProvider.overrideWithValue(fakeDio(http)),
            kvStoreProvider.overrideWithValue(MemoryKvStore()),
            environmentProbeProvider.overrideWithValue(_probe),
            initialEnvironmentsProvider.overrideWithValue(
              EnvironmentsState(
                items: [_env('e', 'Acme test', EnvironmentKind.test, _testUrl)],
                activeId: 'e',
              ),
            ),
          ],
        );
        addTearDown(c.dispose);
        final router = GoRouter(
          initialLocation: '/orders',
          routes: [
            ShellRoute(
              builder: (_, s, child) =>
                  AppShell(location: s.matchedLocation, child: child),
              routes: [
                GoRoute(path: '/orders', builder: (_, _) => const Text('LIST')),
              ],
            ),
          ],
        );
        await tester.pumpWidget(
          UncontrolledProviderScope(
            container: c,
            child: MaterialApp.router(
              routerConfig: router,
              locale: const Locale('en'),
              localizationsDelegates: AppLocalizations.localizationsDelegates,
              supportedLocales: AppLocalizations.supportedLocales,
            ),
          ),
        );
        await tester.pumpAndSettle();
        expect(find.text('TEST · Acme test'), findsOneWidget);
        expect(find.text('LIST'), findsOneWidget);
      });
    }
  });

  group('switcher', () {
    testWidgets('adding takes the kind from the server and switches', (
      tester,
    ) async {
      final c = _container();
      await tester.pumpWidget(
        _app(c, const Scaffold(body: EnvironmentBanner())),
      );
      await tester.tap(find.byType(EnvironmentBanner));
      await tester.pumpAndSettle();
      expect(find.text('Acme test'), findsOneWidget);

      await tester.tap(find.byKey(const ValueKey('env-add')));
      await tester.pumpAndSettle();
      // Not addable before the server was asked.
      expect(
        tester
            .widget<FilledButton>(find.byKey(const ValueKey('env-add-confirm')))
            .onPressed,
        isNull,
      );
      _servers['http://localhost:8083'] = const EnvironmentProbe(
        name: 'Practice',
        kind: EnvironmentKind.tutorial,
        dbName: 'practice',
        version: '0.1.0',
      );
      addTearDown(() => _servers.remove('http://localhost:8083'));
      await tester.enterText(
        find.byKey(const ValueKey('env-url')),
        'http://localhost:8083/',
      );
      await tester.tap(find.byKey(const ValueKey('env-check')));
      await tester.pumpAndSettle();
      expect(find.byKey(const ValueKey('env-detected-kind')), findsOneWidget);
      // The name is proposed from the server.
      expect(find.widgetWithText(TextField, 'Practice'), findsOneWidget);

      await tester.tap(find.byKey(const ValueKey('env-add-confirm')));
      await tester.pumpAndSettle();

      final state = c.read(environmentsProvider);
      expect(state.items, hasLength(3));
      expect(state.active.kind, EnvironmentKind.tutorial);
      expect(state.active.serverUrl, 'http://localhost:8083');
      expect(c.read(dioProvider).options.baseUrl, 'http://localhost:8083');
      expect(find.text('TUTORIAL · Practice'), findsOneWidget);
    });

    testWidgets('an unreachable server cannot be added', (tester) async {
      final c = _container();
      await tester.pumpWidget(
        _app(c, const Scaffold(body: EnvironmentBanner())),
      );
      await tester.tap(find.byType(EnvironmentBanner));
      await tester.pumpAndSettle();
      await tester.tap(find.byKey(const ValueKey('env-add')));
      await tester.pumpAndSettle();
      await tester.enterText(
        find.byKey(const ValueKey('env-url')),
        'http://nowhere.invalid',
      );
      await tester.tap(find.byKey(const ValueKey('env-check')));
      await tester.pumpAndSettle();
      expect(find.textContaining('Could not read the environment'), findsOne);
      expect(
        tester
            .widget<FilledButton>(find.byKey(const ValueKey('env-add-confirm')))
            .onPressed,
        isNull,
      );
    });

    testWidgets('tapping another environment switches to it', (tester) async {
      final c = _container();
      await tester.pumpWidget(
        _app(c, const Scaffold(body: EnvironmentBanner())),
      );
      await tester.tap(find.byType(EnvironmentBanner));
      await tester.pumpAndSettle();
      await tester.tap(find.byKey(const ValueKey('env-tile-env-2')));
      await tester.pumpAndSettle();
      expect(c.read(environmentsProvider).activeId, 'env-2');
      expect(c.read(dioProvider).options.baseUrl, _testUrl);
      expect(find.text('TEST · Acme test'), findsOneWidget);
    });

    testWidgets('removing asks first and keeps the active one', (tester) async {
      final c = _container();
      await tester.pumpWidget(
        _app(c, const Scaffold(body: EnvironmentBanner())),
      );
      await tester.tap(find.byType(EnvironmentBanner));
      await tester.pumpAndSettle();
      // The active environment has no remove button.
      expect(find.byKey(const ValueKey('env-remove-env-1')), findsNothing);
      await tester.tap(find.byKey(const ValueKey('env-remove-env-2')));
      await tester.pumpAndSettle();
      await tester.tap(find.byKey(const ValueKey('env-remove-confirm')));
      await tester.pumpAndSettle();
      expect(c.read(environmentsProvider).items.map((e) => e.id), ['env-1']);
    });

    testWidgets('the login page offers the switcher and shows the banner', (
      tester,
    ) async {
      final c = _container();
      await tester.pumpWidget(_app(c, const LoginPage()));
      await tester.pump();
      expect(find.text('LIVE · Acme production'), findsOneWidget);
      await tester.tap(find.byKey(const ValueKey('login-environments')));
      await tester.pumpAndSettle();
      await tester.tap(find.byKey(const ValueKey('env-tile-env-2')));
      await tester.pumpAndSettle();
      // The server field follows the chosen environment.
      expect(find.widgetWithText(TextField, _testUrl), findsOneWidget);
      expect(find.text('TEST · Acme test'), findsOneWidget);
    });
  });

  group('unsaved tabs', () {
    Future<ProviderContainer> open(
      WidgetTester tester, {
      required bool dirty,
    }) async {
      final c = _container();
      final ws = c.read(workspaceProvider.notifier);
      final id = ws.open('/orders/o1');
      ws.open('/quotations/q1');
      if (dirty) ws.dirtyOf(id).value = true;
      await tester.pumpWidget(
        _app(c, const Scaffold(body: EnvironmentBanner())),
      );
      await tester.tap(find.byType(EnvironmentBanner));
      await tester.pumpAndSettle();
      return c;
    }

    testWidgets('no dirty tab: switches at once', (tester) async {
      final c = await open(tester, dirty: false);
      await tester.tap(find.byKey(const ValueKey('env-tile-env-2')));
      await tester.pumpAndSettle();
      expect(find.byKey(const ValueKey('env-unsaved-discard')), findsNothing);
      expect(c.read(environmentsProvider).activeId, 'env-2');
      expect(c.read(workspaceProvider).tabs, isEmpty);
    });

    testWidgets('a dirty tab asks first; Cancel keeps everything', (
      tester,
    ) async {
      final c = await open(tester, dirty: true);
      await tester.tap(find.byKey(const ValueKey('env-tile-env-2')));
      await tester.pumpAndSettle();
      expect(find.textContaining('1 open tab has unsaved changes'), findsOne);
      expect(find.text('Discard and switch'), findsOneWidget);
      await tester.tap(find.byKey(const ValueKey('env-unsaved-cancel')));
      await tester.pumpAndSettle();
      expect(c.read(environmentsProvider).activeId, 'env-1');
      expect(c.read(dioProvider).options.baseUrl, _liveUrl);
      expect(c.read(workspaceProvider).tabs, hasLength(2));
    });

    testWidgets('confirming switches and closes the tabs', (tester) async {
      final c = await open(tester, dirty: true);
      await tester.tap(find.byKey(const ValueKey('env-tile-env-2')));
      await tester.pumpAndSettle();
      await tester.tap(find.byKey(const ValueKey('env-unsaved-discard')));
      await tester.pumpAndSettle();
      expect(c.read(environmentsProvider).activeId, 'env-2');
      expect(c.read(dioProvider).options.baseUrl, _testUrl);
      expect(c.read(workspaceProvider).tabs, isEmpty);
    });

    testWidgets('adding an environment confirms too', (tester) async {
      final c = await open(tester, dirty: true);
      _servers['http://localhost:8084'] = const EnvironmentProbe(
        name: 'Practice',
        kind: EnvironmentKind.tutorial,
        dbName: 'practice',
        version: '0.1.0',
      );
      addTearDown(() => _servers.remove('http://localhost:8084'));
      await tester.tap(find.byKey(const ValueKey('env-add')));
      await tester.pumpAndSettle();
      await tester.enterText(
        find.byKey(const ValueKey('env-url')),
        'http://localhost:8084',
      );
      await tester.tap(find.byKey(const ValueKey('env-check')));
      await tester.pumpAndSettle();
      await tester.tap(find.byKey(const ValueKey('env-add-confirm')));
      await tester.pumpAndSettle();
      // Asked first: nothing added or closed yet.
      expect(find.byKey(const ValueKey('env-unsaved-discard')), findsOneWidget);
      expect(c.read(environmentsProvider).items, hasLength(2));
      expect(c.read(workspaceProvider).tabs, hasLength(2));
      await tester.tap(find.byKey(const ValueKey('env-unsaved-cancel')));
      await tester.pumpAndSettle();
      expect(c.read(environmentsProvider).items, hasLength(2));
      await tester.tap(find.byKey(const ValueKey('env-add-confirm')));
      await tester.pumpAndSettle();
      await tester.tap(find.byKey(const ValueKey('env-unsaved-discard')));
      await tester.pumpAndSettle();
      expect(c.read(environmentsProvider).items, hasLength(3));
      expect(c.read(environmentsProvider).active.name, 'Practice');
      expect(c.read(workspaceProvider).tabs, isEmpty);
    });
  });
}
