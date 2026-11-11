import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:tradocraft_app/shared/entity_list.dart';
import 'package:tradocraft_app/l10n/app_localizations.dart';

void main() {
  testWidgets('loads, searches and pages', (tester) async {
    tester.view.physicalSize = const Size(1200, 800);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.reset);

    final all = List.generate(
      60,
      (i) => 'Item ${i.toString().padLeft(2, '0')}',
    );
    final queries = <String?>[];
    await tester.pumpWidget(
      MaterialApp(
        locale: const Locale('en'),
        localizationsDelegates: AppLocalizations.localizationsDelegates,
        supportedLocales: AppLocalizations.supportedLocales,
        home: Scaffold(
          body: EntityList<String>(
            title: 'Things',
            load: (q, offset, limit) async {
              queries.add(q);
              final hits = all
                  .where((s) => q == null || s.contains(q))
                  .toList();
              return (hits.skip(offset).take(limit).toList(), hits.length);
            },
            columns: [EntityColumn('Name', (s) => s)],
          ),
        ),
      ),
    );
    await tester.pumpAndSettle();
    expect(find.text('50 of 60'), findsOneWidget);

    await tester.scrollUntilVisible(
      find.text('Load more'),
      500,
      scrollable: find.byType(Scrollable).last,
    );
    await tester.tap(find.text('Load more'));
    await tester.pumpAndSettle();
    expect(find.text('60 of 60'), findsOneWidget);

    await tester.enterText(find.byType(TextField), 'Item 5');
    await tester.pump(const Duration(milliseconds: 400));
    await tester.pumpAndSettle();
    expect(queries.last, 'Item 5');
    expect(find.text('10 of 10'), findsOneWidget);
  });
}
