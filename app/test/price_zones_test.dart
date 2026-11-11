import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:tradocraft_app/app/api.dart';
import 'package:tradocraft_app/features/admin/price_zones_page.dart';
import 'package:tradocraft_app/l10n/app_localizations.dart';

import 'fake_http.dart';

Future<void> _pump(WidgetTester tester, FakeHttp http) async {
  tester.view.physicalSize = const Size(1400, 2000);
  tester.view.devicePixelRatio = 1;
  addTearDown(tester.view.reset);
  await tester.pumpWidget(
    ProviderScope(
      overrides: [dioProvider.overrideWithValue(fakeDio(http))],
      child: MaterialApp(
        locale: const Locale('en'),
        localizationsDelegates: AppLocalizations.localizationsDelegates,
        supportedLocales: AppLocalizations.supportedLocales,
        home: Scaffold(
          body: Consumer(
            builder: (context, ref, _) {
              ref.watch(meProvider);
              return const PriceZonesPage();
            },
          ),
        ),
      ),
    ),
  );
  await tester.pumpAndSettle();
}

Map<String, Object?> _zone(String code, String name, int sort) => {
  'id': 'id-$code',
  'code': code,
  'name': name,
  'sort_order': sort,
};

void main() {
  testWidgets('lists, creates, edits and deletes price zones', (tester) async {
    final zones = [_zone('Z1', 'Retail', 1), _zone('Z2', 'Dealer', 2)];
    final http = FakeHttp({
      'GET /api/v1/me': (_) => meJson(['prices.write']),
      'GET /api/v1/price-zones': (_) => zones,
      'POST /api/v1/price-zones': (b) {
        final m = b! as Map;
        zones.add(_zone(m['code'] as String, m['name'] as String, 3));
        return zones.last;
      },
      'PUT /api/v1/price-zones/id-Z1': (_) => zones.first,
      'DELETE /api/v1/price-zones/id-Z2': (_) {
        zones.removeWhere((z) => z['code'] == 'Z2');
        return null;
      },
    });
    await _pump(tester, http);
    expect(find.text('Z1  Retail'), findsOneWidget);
    expect(find.text('Z2  Dealer'), findsOneWidget);

    await tester.tap(find.byKey(const ValueKey('zone-new')));
    await tester.pumpAndSettle();
    await tester.tap(find.byKey(const ValueKey('zone-save')));
    await tester.pumpAndSettle();
    expect(find.text('Code and name are required.'), findsOneWidget);
    await tester.enterText(find.byKey(const ValueKey('zone-code')), 'Z3');
    await tester.enterText(find.byKey(const ValueKey('zone-name')), 'Export');
    await tester.tap(find.byKey(const ValueKey('zone-save')));
    await tester.pumpAndSettle();
    expect(http.bodies['POST /api/v1/price-zones'], {
      'code': 'Z3',
      'name': 'Export',
      'sort_order': 0,
    });
    expect(find.text('Z3  Export'), findsOneWidget);

    await tester.tap(find.byKey(const ValueKey('zone-edit-Z1')));
    await tester.pumpAndSettle();
    await tester.enterText(find.byKey(const ValueKey('zone-name')), 'Shop');
    await tester.tap(find.byKey(const ValueKey('zone-save')));
    await tester.pumpAndSettle();
    expect(
      (http.bodies['PUT /api/v1/price-zones/id-Z1']! as Map)['name'],
      'Shop',
    );

    await tester.tap(find.byKey(const ValueKey('zone-del-Z2')));
    await tester.pumpAndSettle();
    await tester.tap(find.byKey(const ValueKey('zone-delete-confirm')));
    await tester.pumpAndSettle();
    expect(http.requests, contains('DELETE /api/v1/price-zones/id-Z2'));
    expect(find.text('Z2  Dealer'), findsNothing);
  });

  testWidgets('a zone in use shows the server refusal and stays', (
    tester,
  ) async {
    final http = FakeHttp({
      'GET /api/v1/me': (_) => meJson(['prices.write']),
      'GET /api/v1/price-zones': (_) => [_zone('Z1', 'Retail', 1)],
      'DELETE /api/v1/price-zones/id-Z1': (_) => FakeStatus(409, {
        'error': 'conflict',
        'message': 'price zone Z1 is in use',
      }),
    });
    await _pump(tester, http);
    await tester.tap(find.byKey(const ValueKey('zone-del-Z1')));
    await tester.pumpAndSettle();
    await tester.tap(find.byKey(const ValueKey('zone-delete-confirm')));
    await tester.pumpAndSettle();
    expect(find.textContaining('is in use'), findsOneWidget);
    expect(find.text('Z1  Retail'), findsOneWidget);
  });

  testWidgets('without prices.write the zones are read-only', (tester) async {
    final http = FakeHttp({
      'GET /api/v1/me': (_) => meJson(['products.read']),
      'GET /api/v1/price-zones': (_) => [_zone('Z1', 'Retail', 1)],
    });
    await _pump(tester, http);
    expect(find.text('Z1  Retail'), findsOneWidget);
    expect(find.byKey(const ValueKey('zone-new')), findsNothing);
    expect(find.byKey(const ValueKey('zone-del-Z1')), findsNothing);
  });
}
