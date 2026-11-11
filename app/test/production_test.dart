import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:tradocraft_app/app/api.dart';
import 'package:tradocraft_app/features/production/divisions_page.dart';
import 'package:tradocraft_app/features/production/order_production.dart';
import 'package:tradocraft_app/features/production/production_page.dart';
import 'package:tradocraft_app/features/production/production_screen.dart';
import 'package:tradocraft_app/features/production/release_page.dart';
import 'package:tradocraft_app/l10n/app_localizations.dart';

import 'fake_http.dart';

Map<String, Object?> _component({
  String id = 'c1',
  String code = 'PROFILE-40',
  String tracking = 'none',
  String required = '14.3',
  String reserved = '14.3',
  bool manufactured = false,
  bool short = false,
}) => {
  'id': id,
  'position': 1,
  'component_product_id': 'p-$id',
  'component_code': code,
  'component_name': 'Steel profile',
  'unit': 'm',
  'tracking': tracking,
  'required_quantity': required,
  'reserved_quantity': reserved,
  'consumed_quantity': '0',
  'scrap_quantity': '0',
  'manufactured': manufactured,
  'short': short,
};

Map<String, Object?> _summary({
  String id = 'po1',
  String status = 'released',
  int short = 0,
  bool orderCancelled = false,
  String tracking = 'none',
}) => {
  'id': id,
  'number': 'MO2099-00001',
  'product_id': 'p1',
  'product_code': 'FRAME-A',
  'product_name': 'Frame assembly',
  'unit': 'pcs',
  'product_tracking': tracking,
  'quantity': '2',
  'bom_revision': 1,
  'division_id': 'd1',
  'division_name': 'Welding',
  'order_id': 'o1',
  'order_number': 'SO2099-00001',
  'customer_name': 'Alpha Installaties NV',
  'priority': 'expedite',
  'order_cancelled': orderCancelled,
  'status': status,
  'planned_ready_date': '2099-07-01',
  'short_components': short,
  'created_at': '2099-01-01T10:00:00Z',
};

Map<String, Object?> _order({
  String status = 'released',
  List<Map<String, Object?>>? components,
  bool orderCancelled = false,
  String tracking = 'none',
}) => {
  ..._summary(
    status: status,
    orderCancelled: orderCancelled,
    tracking: tracking,
    short: (components ?? [_component()])
        .where((c) => c['short'] == true)
        .length,
  ),
  'order_line_id': 'ol1',
  'parent_order_id': null,
  'started_at': null,
  'completed_at': null,
  'completed_quantity': null,
  'notes': '',
  'cancel_reason': null,
  'components': components ?? [_component()],
  'outputs': <Object?>[],
};

Map<String, Object?> _division({String id = 'd1', String code = 'WELD'}) => {
  'id': id,
  'code': code,
  'name': 'Welding',
  'sort_order': 1,
  'active': true,
  'open_orders': 2,
};

Map<String, Object?> _warehouse() => {
  'id': 'w1',
  'code': 'WH1',
  'name': 'Main warehouse',
  'kind': 'warehouse',
  'active': true,
  'assigned_user_id': null,
  'assigned_user_name': null,
  'locations': [
    {
      'id': 'l1',
      'warehouse_id': 'w1',
      'code': 'MAIN',
      'name': '',
      'active': true,
    },
  ],
};

Map<String, Object?> _toRelease() => {
  'order_id': 'o1',
  'order_number': 'SO2099-00001',
  'customer_name': 'Alpha Installaties NV',
  'priority': 'normal',
  'order_line_id': 'ol1',
  'product_id': 'p1',
  'product_code': 'FRAME-A',
  'product_name': 'Frame assembly',
  'ordered': '2',
  'planned': '0',
  'to_build': '2',
};

Future<void> _pump(WidgetTester tester, FakeHttp http, Widget home) async {
  tester.view.physicalSize = const Size(1400, 2000);
  tester.view.devicePixelRatio = 1;
  addTearDown(tester.view.reset);
  final router = GoRouter(
    routes: [
      GoRoute(
        path: '/',
        builder: (_, _) => Scaffold(body: home),
      ),
      GoRoute(
        path: '/production/:id',
        builder: (_, s) =>
            Scaffold(body: Text('production ${s.pathParameters['id']}')),
      ),
      GoRoute(
        path: '/production/release',
        builder: (_, _) => const Scaffold(body: Text('release page')),
      ),
      GoRoute(
        path: '/production/divisions',
        builder: (_, _) => const Scaffold(body: Text('divisions page')),
      ),
      GoRoute(
        path: '/orders/:id',
        builder: (_, s) =>
            Scaffold(body: Text('order ${s.pathParameters['id']}')),
      ),
    ],
  );
  await tester.pumpWidget(
    ProviderScope(
      overrides: [dioProvider.overrideWithValue(fakeDio(http))],
      child: MaterialApp.router(
        locale: const Locale('en'),
        localizationsDelegates: AppLocalizations.localizationsDelegates,
        supportedLocales: AppLocalizations.supportedLocales,
        routerConfig: router,
      ),
    ),
  );
  await tester.pumpAndSettle();
}

const _planner = ['production.read', 'production.write', 'production.run'];

FakeHttp _screenHttp(
  List<String> perms, {
  Map<String, Object?>? order,
  Map<String, Object? Function(Object?)> extra = const {},
}) => FakeHttp({
  'GET /api/v1/me': (_) => meJson(perms),
  'GET /api/v1/production-orders/po1': (_) => order ?? _order(),
  'GET /api/v1/production/divisions': (_) => [_division()],
  'GET /api/v1/inventory/warehouses': (_) => [_warehouse()],
  ...extra,
});

void main() {
  testWidgets('the queue shows the urgent orders and flags what is short', (
    tester,
  ) async {
    final http = FakeHttp({
      'GET /api/v1/me': (_) =>
          meJson(['production.read', 'production.admin', 'production.write']),
      'GET /api/v1/production/divisions': (_) => [_division()],
      'GET /api/v1/production-orders': (_) => {
        'items': [_summary(short: 1)],
        'total': 1,
      },
    });
    await _pump(tester, http, const ProductionQueuePage());
    expect(find.text('MO2099-00001'), findsWidgets);
    expect(find.textContaining('FRAME-A'), findsWidgets);
    expect(find.byTooltip('short of stock'), findsOneWidget);
    expect(find.byKey(const ValueKey('to-release')), findsOneWidget);
    expect(find.byKey(const ValueKey('divisions')), findsOneWidget);
    expect(find.text('Build for stock'), findsOneWidget);
  });

  testWidgets('a reader sees the queue without planning or admin buttons', (
    tester,
  ) async {
    final http = FakeHttp({
      'GET /api/v1/me': (_) => meJson(['production.read']),
      'GET /api/v1/production/divisions': (_) => [_division()],
      'GET /api/v1/production-orders': (_) => {
        'items': <Object?>[],
        'total': 0,
      },
    });
    await _pump(tester, http, const ProductionQueuePage());
    expect(find.byKey(const ValueKey('divisions')), findsNothing);
    expect(find.text('Build for stock'), findsNothing);
  });

  testWidgets('the planner releases a line from the to-release list', (
    tester,
  ) async {
    final http = FakeHttp({
      'GET /api/v1/me': (_) => meJson(_planner),
      'GET /api/v1/production/to-release': (_) => [_toRelease()],
      'POST /api/v1/production-orders': (_) => _order(),
    });
    await _pump(tester, http, const ReleasePage());
    expect(find.textContaining('FRAME-A'), findsOneWidget);
    await tester.tap(find.byKey(const ValueKey('release-button-ol1')));
    await tester.pumpAndSettle();
    final sent = http.bodies['POST /api/v1/production-orders']! as Map;
    expect(sent['order_id'], 'o1');
    expect(sent['order_line_id'], 'ol1');
    expect(
      find.text('production po1'),
      findsOneWidget,
      reason: 'opens the new order',
    );
  });

  testWidgets('with nothing to release the list says so', (tester) async {
    final http = FakeHttp({
      'GET /api/v1/me': (_) => meJson(_planner),
      'GET /api/v1/production/to-release': (_) => <Object?>[],
    });
    await _pump(tester, http, const ReleasePage());
    expect(find.textContaining('Nothing to release'), findsOneWidget);
  });

  testWidgets(
    'the order screen saves division, date and notes and starts the order',
    (tester) async {
      final http = _screenHttp(
        _planner,
        extra: {
          'PUT /api/v1/production-orders/po1': (_) => _order(),
          'POST /api/v1/production-orders/po1/start': (_) =>
              _order(status: 'in_progress'),
        },
      );
      await _pump(tester, http, const ProductionScreen(id: 'po1'));
      expect(find.textContaining('PROFILE-40'), findsOneWidget);
      await tester.tap(find.byKey(const ValueKey('save-production')));
      await tester.pumpAndSettle();
      final sent = http.bodies['PUT /api/v1/production-orders/po1']! as Map;
      expect(sent['division_id'], 'd1');
      expect(
        (sent['planned_ready_date'] as String).startsWith('2099-07-01'),
        isTrue,
      );

      await tester.tap(find.byKey(const ValueKey('start-production')));
      await tester.pumpAndSettle();
      expect(
        http.requests,
        contains('POST /api/v1/production-orders/po1/start'),
      );
      expect(find.byKey(const ValueKey('start-production')), findsNothing);
    },
  );

  testWidgets(
    'a short manufactured component can be built and reserved again',
    (tester) async {
      final short = _component(
        id: 'c9',
        code: 'SUB-1',
        required: '3',
        reserved: '1',
        manufactured: true,
        short: true,
      );
      final http = _screenHttp(
        _planner,
        order: _order(components: [short]),
        extra: {
          'POST /api/v1/production-orders': (_) => {..._order(), 'id': 'child'},
          // Still short after trying again: nothing arrived in stock.
          'POST /api/v1/production-orders/po1/reserve': (_) =>
              _order(components: [short]),
        },
      );
      await _pump(tester, http, const ProductionScreen(id: 'po1'));
      expect(find.text('short of stock'), findsNothing);
      expect(find.textContaining('short of stock'), findsWidgets);

      await tester.tap(find.byKey(const ValueKey('reserve-again')));
      await tester.pumpAndSettle();
      expect(
        http.requests,
        contains('POST /api/v1/production-orders/po1/reserve'),
      );

      await tester.tap(find.byKey(const ValueKey('build-c9')));
      await tester.pumpAndSettle();
      final sent = http.bodies['POST /api/v1/production-orders']! as Map;
      expect(sent['product_id'], 'p-c9');
      expect(sent['quantity'], '2');
      expect(sent['parent_order_id'], 'po1');
      expect(find.text('production child'), findsOneWidget);
    },
  );

  testWidgets('cancelling asks for a reason', (tester) async {
    final http = _screenHttp(
      _planner,
      extra: {
        'POST /api/v1/production-orders/po1/cancel': (_) =>
            _order(status: 'cancelled'),
      },
    );
    await _pump(tester, http, const ProductionScreen(id: 'po1'));
    await tester.tap(find.byKey(const ValueKey('cancel-production')));
    await tester.pumpAndSettle();
    await tester.enterText(find.byType(TextField).last, 'customer changed it');
    await tester.tap(find.text('OK'));
    await tester.pumpAndSettle();
    final sent =
        http.bodies['POST /api/v1/production-orders/po1/cancel']! as Map;
    expect(sent['reason'], 'customer changed it');
    expect(find.byKey(const ValueKey('cancel-production')), findsNothing);
  });

  testWidgets('a cancelled sales order is flagged and cannot be completed', (
    tester,
  ) async {
    final http = _screenHttp(_planner, order: _order(orderCancelled: true));
    await _pump(tester, http, const ProductionScreen(id: 'po1'));
    expect(find.byKey(const ValueKey('order-cancelled')), findsOneWidget);
    expect(find.byKey(const ValueKey('complete-production')), findsNothing);
    expect(find.byKey(const ValueKey('cancel-production')), findsOneWidget);
  });

  testWidgets('a reader sees the order but has no actions', (tester) async {
    final http = _screenHttp(['production.read']);
    await _pump(tester, http, const ProductionScreen(id: 'po1'));
    for (final k in [
      'save-production',
      'start-production',
      'complete-production',
      'cancel-production',
    ]) {
      expect(find.byKey(ValueKey(k)), findsNothing, reason: k);
    }
    expect(find.textContaining('PROFILE-40'), findsOneWidget);
  });

  testWidgets('completing sends the output and the actual consumption', (
    tester,
  ) async {
    final http = _screenHttp(
      _planner,
      extra: {
        'POST /api/v1/production-orders/po1/complete': (_) =>
            _order(status: 'completed'),
      },
    );
    await _pump(tester, http, const ProductionScreen(id: 'po1'));
    await tester.tap(find.byKey(const ValueKey('complete-production')));
    await tester.pumpAndSettle();
    await tester.enterText(find.byKey(const ValueKey('output-quantity')), '2');
    // The location is a dropdown of the stock locations.
    await tester.tap(find.byType(DropdownButtonFormField<String>).last);
    await tester.pumpAndSettle();
    await tester.tap(find.text('WH1/MAIN').last);
    await tester.pumpAndSettle();
    await tester.enterText(find.byKey(const ValueKey('scrap-c1')), '1');
    await tester.pumpAndSettle();
    // 14.3 used + 1 scrapped is more than the 14.3 reserved: the rest needs a place.
    expect(find.byKey(const ValueKey('extra-c1')), findsOneWidget);
    await tester.tap(find.byKey(const ValueKey('complete-save')));
    await tester.pumpAndSettle();
    final sent =
        http.bodies['POST /api/v1/production-orders/po1/complete']! as Map;
    expect(sent['output']['quantity'], '2');
    expect(sent['output']['location_id'], 'l1');
    final c = (sent['components'] as List).first as Map;
    expect(c['component_id'], 'c1');
    expect(c['consumed_quantity'], '14.3');
    expect(c['scrap_quantity'], '1');
    expect(
      find.byKey(const ValueKey('complete-save')),
      findsNothing,
      reason: 'closed',
    );
    expect(find.text('Production order completed'), findsOneWidget);
  });

  testWidgets('a refused completion keeps the dialog open with the reason', (
    tester,
  ) async {
    final http = _screenHttp(
      _planner,
      extra: {
        'POST /api/v1/production-orders/po1/complete': (_) => const FakeStatus(
          422,
          {'error': 'validation', 'message': 'a lot number is needed'},
        ),
      },
    );
    await _pump(tester, http, const ProductionScreen(id: 'po1'));
    await tester.tap(find.byKey(const ValueKey('complete-production')));
    await tester.pumpAndSettle();
    await tester.tap(find.byType(DropdownButtonFormField<String>).last);
    await tester.pumpAndSettle();
    await tester.tap(find.text('WH1/MAIN').last);
    await tester.pumpAndSettle();
    await tester.tap(find.byKey(const ValueKey('complete-save')));
    await tester.pumpAndSettle();
    expect(find.textContaining('a lot number is needed'), findsOneWidget);
    expect(find.byKey(const ValueKey('complete-save')), findsOneWidget);
  });

  testWidgets(
    'lot and serial output ask for their numbers, serial components for their units',
    (tester) async {
      final serial = _component(
        id: 'c2',
        code: 'PUMP-05',
        tracking: 'serial',
        required: '2',
        reserved: '2',
      );
      final http = _screenHttp(
        _planner,
        order: _order(components: [serial], tracking: 'serial'),
        extra: {
          'GET /api/v1/production-orders/po1/components/c2/serial-units': (_) =>
              [
                {
                  'id': 'u1',
                  'serial_number': 'P-1',
                  'location_id': 'l1',
                  'location': 'WH1/MAIN',
                },
                {
                  'id': 'u2',
                  'serial_number': 'P-2',
                  'location_id': 'l1',
                  'location': 'WH1/MAIN',
                },
              ],
          'POST /api/v1/production-orders/po1/complete': (_) =>
              _order(status: 'completed'),
        },
      );
      await _pump(tester, http, const ProductionScreen(id: 'po1'));
      await tester.tap(find.byKey(const ValueKey('complete-production')));
      await tester.pumpAndSettle();
      expect(find.byKey(const ValueKey('output-serials')), findsOneWidget);
      expect(
        find.byKey(const ValueKey('scrap-c2')),
        findsNothing,
        reason: 'units are scrapped separately',
      );
      await tester.enterText(
        find.byKey(const ValueKey('output-quantity')),
        '1',
      );
      await tester.enterText(find.byKey(const ValueKey('consumed-c2')), '2');
      await tester.enterText(
        find.byKey(const ValueKey('output-serials')),
        'U-1\n',
      );
      await tester.tap(find.byType(DropdownButtonFormField<String>).last);
      await tester.pumpAndSettle();
      await tester.tap(find.text('WH1/MAIN').last);
      await tester.pumpAndSettle();

      await tester.tap(find.byKey(const ValueKey('units-c2')));
      await tester.pumpAndSettle();
      await tester.tap(find.byKey(const ValueKey('unit-u1')));
      await tester.tap(find.byKey(const ValueKey('unit-u2')));
      await tester.tap(find.byKey(const ValueKey('units-save')));
      await tester.pumpAndSettle();
      expect(find.textContaining('P-1, P-2'), findsOneWidget);

      await tester.tap(find.byKey(const ValueKey('complete-save')));
      await tester.pumpAndSettle();
      final sent =
          http.bodies['POST /api/v1/production-orders/po1/complete']! as Map;
      expect(sent['output']['serial_numbers'], ['U-1']);
      final c = (sent['components'] as List).first as Map;
      expect(c['serial_unit_ids'], ['u1', 'u2']);
      expect(c.containsKey('scrap_quantity'), isFalse);
    },
  );

  testWidgets('a lot-tracked output asks for its lot', (tester) async {
    final http = _screenHttp(
      _planner,
      order: _order(tracking: 'lot'),
      extra: {
        'POST /api/v1/production-orders/po1/complete': (_) =>
            _order(status: 'completed'),
      },
    );
    await _pump(tester, http, const ProductionScreen(id: 'po1'));
    await tester.tap(find.byKey(const ValueKey('complete-production')));
    await tester.pumpAndSettle();
    expect(find.byKey(const ValueKey('output-lot')), findsOneWidget);
    await tester.enterText(find.byKey(const ValueKey('output-lot')), 'F-001');
    await tester.tap(find.byType(DropdownButtonFormField<String>).last);
    await tester.pumpAndSettle();
    await tester.tap(find.text('WH1/MAIN').last);
    await tester.pumpAndSettle();
    await tester.tap(find.byKey(const ValueKey('complete-save')));
    await tester.pumpAndSettle();
    final sent =
        http.bodies['POST /api/v1/production-orders/po1/complete']! as Map;
    expect(sent['output']['lot_number'], 'F-001');
  });

  testWidgets(
    'the order lists its production orders and releases what is left',
    (tester) async {
      final http = FakeHttp({
        'GET /api/v1/me': (_) => meJson(_planner),
        'GET /api/v1/production-orders': (_) => {
          'items': [_summary(short: 1)],
          'total': 1,
        },
        'GET /api/v1/production/to-release': (_) => [_toRelease()],
        'POST /api/v1/production-orders': (_) => _order(),
      });
      var changed = 0;
      await _pump(
        tester,
        http,
        OrderProduction(
          orderId: 'o1',
          orderStatus: 'confirmed',
          onChanged: () => changed++,
        ),
      );
      final list = http.requests.indexOf('GET /api/v1/production-orders');
      expect(http.queries[list]['order_id'], 'o1');
      expect(find.textContaining('MO2099-00001'), findsOneWidget);
      await tester.tap(find.byKey(const ValueKey('release-order')));
      await tester.pumpAndSettle();
      expect(http.requests, contains('POST /api/v1/production-orders'));
      expect(
        changed,
        1,
        reason: 'the order refreshes: its status follows production',
      );
    },
  );

  testWidgets(
    'releasing with nothing left says so, and a reader cannot release',
    (tester) async {
      final http = FakeHttp({
        'GET /api/v1/me': (_) => meJson(_planner),
        'GET /api/v1/production-orders': (_) => {
          'items': <Object?>[],
          'total': 0,
        },
        'GET /api/v1/production/to-release': (_) => <Object?>[],
      });
      await _pump(
        tester,
        http,
        const OrderProduction(orderId: 'o1', orderStatus: 'confirmed'),
      );
      await tester.tap(find.byKey(const ValueKey('release-order')));
      await tester.pumpAndSettle();
      expect(find.textContaining('Nothing to release'), findsOneWidget);
      expect(http.requests, isNot(contains('POST /api/v1/production-orders')));

      final reader = FakeHttp({
        'GET /api/v1/me': (_) => meJson(['production.read']),
        'GET /api/v1/production-orders': (_) => {
          'items': <Object?>[],
          'total': 0,
        },
      });
      await _pump(
        tester,
        reader,
        const OrderProduction(orderId: 'o1', orderStatus: 'confirmed'),
      );
      expect(find.byKey(const ValueKey('release-order')), findsNothing);
    },
  );

  testWidgets(
    'the administrator adds and removes divisions; others are told they cannot',
    (tester) async {
      final http = FakeHttp({
        'GET /api/v1/me': (_) =>
            meJson(['production.read', 'production.admin']),
        'GET /api/v1/production/divisions': (_) => [_division()],
        'POST /api/v1/production/divisions': (_) =>
            _division(id: 'd2', code: 'ASM'),
        'DELETE /api/v1/production/divisions/d1': (_) => null,
      });
      await _pump(tester, http, const DivisionsPage());
      expect(find.textContaining('WELD'), findsOneWidget);
      await tester.tap(find.byKey(const ValueKey('add-division')));
      await tester.pumpAndSettle();
      await tester.enterText(
        find.byKey(const ValueKey('division-code')),
        'ASM',
      );
      await tester.enterText(
        find.byKey(const ValueKey('division-name')),
        'Assembly',
      );
      await tester.tap(find.byKey(const ValueKey('division-save')));
      await tester.pumpAndSettle();
      final sent = http.bodies['POST /api/v1/production/divisions']! as Map;
      expect(sent['code'], 'ASM');
      expect(sent['name'], 'Assembly');
      await tester.tap(find.byKey(const ValueKey('delete-d1')));
      await tester.pumpAndSettle();
      expect(http.requests, contains('DELETE /api/v1/production/divisions/d1'));

      final reader = FakeHttp({
        'GET /api/v1/me': (_) => meJson(['production.read']),
        'GET /api/v1/production/divisions': (_) => [_division()],
      });
      await _pump(tester, reader, const DivisionsPage());
      expect(find.byKey(const ValueKey('add-division')), findsNothing);
      expect(find.textContaining('permission'), findsOneWidget);
    },
  );

  testWidgets('an exact fit in decimals never asks for an extra place', (
    tester,
  ) async {
    // 0.1 + 0.2 is more than 0.3 in binary floating point, but exactly 0.3 in decimals.
    final http = _screenHttp(
      _planner,
      order: _order(
        components: [_component(required: '0.3', reserved: '0.3')],
      ),
    );
    await _pump(tester, http, const ProductionScreen(id: 'po1'));
    await tester.tap(find.byKey(const ValueKey('complete-production')));
    await tester.pumpAndSettle();
    await tester.enterText(find.byKey(const ValueKey('consumed-c1')), '0.1');
    await tester.enterText(find.byKey(const ValueKey('scrap-c1')), '0.2');
    await tester.pumpAndSettle();
    expect(find.byKey(const ValueKey('extra-c1')), findsNothing);
    await tester.enterText(find.byKey(const ValueKey('scrap-c1')), '0.3');
    await tester.pumpAndSettle();
    expect(find.byKey(const ValueKey('extra-c1')), findsOneWidget);
  });

  testWidgets('lines released before a failure still refresh the order', (
    tester,
  ) async {
    var posts = 0;
    final second = {..._toRelease(), 'order_line_id': 'ol2'};
    final http = FakeHttp({
      'GET /api/v1/me': (_) => meJson(_planner),
      'GET /api/v1/production-orders': (_) => {
        'items': [_summary()],
        'total': 1,
      },
      'GET /api/v1/production/to-release': (_) => [_toRelease(), second],
      'POST /api/v1/production-orders': (_) => ++posts == 1
          ? _order()
          : const FakeStatus(409, {
              'error': 'conflict',
              'message': 'no active BOM',
            }),
    });
    var changed = 0;
    await _pump(
      tester,
      http,
      OrderProduction(
        orderId: 'o1',
        orderStatus: 'confirmed',
        onChanged: () => changed++,
      ),
    );
    final loads = http.requests
        .where((r) => r == 'GET /api/v1/production-orders')
        .length;
    await tester.tap(find.byKey(const ValueKey('release-order')));
    await tester.pumpAndSettle();
    expect(posts, 2);
    expect(find.textContaining('no active BOM'), findsOneWidget);
    expect(changed, 1, reason: 'the first line was released');
    expect(
      http.requests.where((r) => r == 'GET /api/v1/production-orders').length,
      loads + 1,
      reason: 'the list is reloaded',
    );
  });

  testWidgets('an order keeps showing its division after it was deactivated', (
    tester,
  ) async {
    final http = _screenHttp(
      _planner,
      extra: {
        'GET /api/v1/production/divisions': (_) => [
          {..._division(), 'active': false},
        ],
      },
    );
    await _pump(tester, http, const ProductionScreen(id: 'po1'));
    expect(
      find.text('Welding'),
      findsOneWidget,
      reason: 'not replaced by "no division"',
    );
  });
}
