import 'dart:convert';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:tradocraft_app/app/api.dart';
import 'package:tradocraft_app/app/environments.dart';
import 'package:tradocraft_app/features/field/offline/kv_store.dart';
import 'package:tradocraft_app/features/tutorial/api_events.dart';
import 'package:tradocraft_app/features/tutorial/controller.dart';
import 'package:tradocraft_app/features/tutorial/missions.dart';
import 'package:tradocraft_app/features/tutorial/simulated_mail.dart';

import 'fake_http.dart';

const _ids = {
  'motorId': 'm1',
  'frameId': 'f1',
  'controllerId': 'c1',
  'cartId': 'k1',
};

/// A container whose stored progress says: these chapters are done, [active] is at step 0, and
/// the story already captured [values].
ProviderContainer _boot(
  FakeHttp http, {
  required List<String> completed,
  required String active,
  Map<String, String> values = const {},
}) {
  final kv = MemoryKvStore()
    ..data['tutorial.v1.e1.dev:a@x.test'] = jsonEncode({
      'completed': completed,
      'active': active,
      'step': 0,
      'values': {..._ids, ...values},
    });
  final c = ProviderContainer(
    retry: (_, _) => null,
    overrides: [
      kvStoreProvider.overrideWithValue(kv),
      tutorialScopeProvider.overrideWithValue(
        const TutorialScope(
          envId: 'e1',
          kind: EnvironmentKind.tutorial,
          who: 'dev:a@x.test',
        ),
      ),
      dioProvider.overrideWith((ref) {
        final d = fakeDio(http);
        d.interceptors.add(ApiEventInterceptor(ref.read(apiEventBusProvider)));
        return d;
      }),
    ],
  );
  addTearDown(c.dispose);
  return c;
}

void main() {
  group('simulated e-mail', () {
    final at = DateTime.utc(2026, 11, 12, 10, 5, 9);

    test('builds an RFC 822 message with CRLF line endings', () {
      final raw = buildRfc822(
        messageId: '<a@example.test>',
        fromName: 'Maya Okafor',
        fromAddress: 'maya@example.test',
        toAddress: 'support@example.test',
        subject: 'Hello',
        body: 'Line one\nLine two',
        date: at,
      );
      expect(raw.contains('\r\n\r\nLine one\r\nLine two\r\n'), isTrue);
      expect(RegExp(r'(?<!\r)\n').hasMatch(raw), isFalse, reason: 'only CRLF');
      expect(raw, contains('Message-ID: <a@example.test>\r\n'));
      expect(raw, contains('Date: Thu, 12 Nov 2026 10:05:09 +0000\r\n'));
      expect(raw, contains('From: Maya Okafor <maya@example.test>\r\n'));
      expect(raw, contains('Content-Transfer-Encoding: 7bit'));
      expect(raw, isNot(contains('In-Reply-To')));
    });

    test('a reply carries In-Reply-To and References', () {
      final raw = buildRfc822(
        messageId: '<b@example.test>',
        inReplyTo: '<a@example.test>',
        fromName: 'M',
        fromAddress: 'm@example.test',
        toAddress: 's@example.test',
        subject: 'Re: x',
        body: 'ok',
        date: at,
      );
      expect(raw, contains('In-Reply-To: <a@example.test>\r\n'));
      expect(raw, contains('References: <a@example.test>\r\n'));
    });

    test('non-ASCII subject and body are encoded and stay ASCII-safe', () {
      final raw = buildRfc822(
        messageId: '<c@example.test>',
        fromName: 'Zoë Müller',
        fromAddress: 'z@example.test',
        toAddress: 's@example.test',
        subject: 'Demande de devis : chariot élevé',
        body: 'Bonjour,\n\nMerci beaucoup, très aimable.',
        date: at,
      );
      expect(raw.codeUnits.every((c) => c < 128), isTrue);
      expect(raw, contains('Content-Transfer-Encoding: base64'));
      final subject = RegExp(r'Subject: (.*)\r\n').firstMatch(raw)!.group(1)!;
      final decoded = [
        for (final m in RegExp(r'=\?UTF-8\?B\?([^?]*)\?=').allMatches(subject))
          utf8.decode(base64.decode(m.group(1)!)),
      ].join();
      expect(decoded, 'Demande de devis : chariot élevé');
      final bodyPart = raw.split('\r\n\r\n')[1].replaceAll('\r\n', '');
      expect(
        utf8.decode(base64.decode(bodyPart)),
        'Bonjour,\r\n\r\nMerci beaucoup, très aimable.',
      );
    });

    test('message ids are unique per run and kind', () {
      final a = newMessageId(at, 'request', 1);
      expect(a, endsWith('@example.test>'));
      expect(newMessageId(at.add(const Duration(microseconds: 1)), 'request', 1), isNot(a));
      expect(newMessageId(at, 'details', 1), isNot(a));
      expect(newMessageId(at, 'request', 2), isNot(a));
    });

    test('every canned mail exists in the five languages, on example.test only', () {
      for (final k in TutorialMailKind.values) {
        for (final l in tutorialMailLanguages) {
          final t = tutorialMailText(k, l);
          expect(t.subject, isNotEmpty);
          expect(t.body, isNotEmpty);
          expect(t.body.contains('@') && !t.body.contains('example.test'), isFalse);
        }
        expect(tutorialMailText(k, 'xx').subject, tutorialMailText(k, 'en').subject);
      }
      expect(tutorialMailText(TutorialMailKind.details, 'nl').body, contains('BE0123456749'));
    });

    test('sendTutorialMail posts message/rfc822 and reads the ticket from the answer', () async {
      final http = FakeHttp({
        'POST /api/v1/helpdesk/inbound': (_) => {
          'ticket_id': 't1',
          'ticket_number': 'T2026-00001',
          'new_ticket': true,
          'duplicate': false,
        },
      });
      final dio = fakeDio(http);
      final r = await sendTutorialMail(
        dio,
        kind: TutorialMailKind.details,
        lang: 'fr',
        inReplyTo: '<first@example.test>',
        ticketNumber: 'T2026-00001',
        now: at,
        salt: 7,
      );
      expect(r.ticketId, 't1');
      expect(r.ticketNumber, 'T2026-00001');
      expect(r.messageId, newMessageId(at, 'details', 7));
      final sent = http.bodies['POST /api/v1/helpdesk/inbound']! as String;
      expect(sent, contains('In-Reply-To: <first@example.test>'));
      expect(sent, contains('[Ticket#T2026-00001]'));
    });
  });

  group('chapter 7: a new customer writes in', () {
    final done = ['welcome', 'product', 'suppliers', 'buying', 'invoices', 'manufacturing', 'prices'];
    test('scripted mails, reply, customer creation', () async {
      final http = FakeHttp({
        'POST /api/v1/helpdesk/inbound': (b) => {
          'ticket_id': 't1',
          'ticket_number': 'T2026-00001',
          'new_ticket': true,
          'duplicate': false,
        },
        'POST /api/v1/tickets/t1/replies': (_) => {'ticket': {'id': 't1'}},
        'POST /api/v1/customers': (b) => {
          'id': 'cu1',
          'name': (b as Map)['name'],
        },
      });
      final c = _boot(http, completed: done, active: 'customer');
      final t = c.read(tutorialProvider.notifier);
      await t.ready;
      await t.resume();
      final dio = c.read(dioProvider);
      int step() => c.read(tutorialProvider).stepIndex;
      t.next(); // needs (no zone found)
      expect(step(), 1);
      await t.runAction();
      expect(step(), 2);
      var v = c.read(tutorialProvider).values;
      expect(v['ticketId'], 't1');
      expect(v['ticketNumber'], 'T2026-00001');
      expect(v['mailId1'], startsWith('<tut.'));
      final first = http.bodies['POST /api/v1/helpdesk/inbound']! as String;
      expect(first, contains('Subject: '));
      expect(first, contains('maya.okafor@example.test'));
      t.routeChanged('/helpdesk');
      expect(step(), 3);
      t.routeChanged('/helpdesk/other');
      expect(step(), 3);
      t.routeChanged('/helpdesk/t1');
      expect(step(), 4);
      await dio.post<Object?>('/api/v1/tickets/t1/replies', data: {'body': 'x'});
      expect(step(), 5);
      await t.runAction();
      expect(step(), 6);
      final second = http.bodies['POST /api/v1/helpdesk/inbound']! as String;
      expect(second, contains('In-Reply-To: ${c.read(tutorialProvider).values['mailId1']}'));
      expect(second, contains('[Ticket#T2026-00001]'));
      expect(second, contains('Content-Transfer-Encoding'));
      t.next(); // readDetails
      expect(step(), 7);
      await dio.post<Object?>('/api/v1/customers', data: {'name': 'Someone Else'});
      expect(step(), 7);
      await dio.post<Object?>(
        '/api/v1/customers',
        data: {'name': 'Brightwheel Logistics BV'},
      );
      expect(step(), 8);
      expect(c.read(tutorialProvider).values['customerId'], 'cu1');
      t.next(); // linked
      t.next();
      expect(c.read(tutorialProvider).completed, contains('customer'));
    });

    test('probes find the zone, the ticket and the customer', () async {
      final http = FakeHttp({
        'GET /api/v1/price-zones': (_) => [
          {'id': 'zx', 'code': 'TUT-EXPORT'},
        ],
        'GET /api/v1/tickets': (_) => {
          'items': [
            {'id': 'tx', 'number': 'T1', 'contact_email': 'other@example.test'},
            {'id': 't1', 'number': 'T2', 'contact_email': 'maya.okafor@example.test'},
          ],
        },
        'GET /api/v1/customers': (_) => {
          'items': [
            {'id': 'cu1', 'name': 'Brightwheel Logistics BV'},
          ],
        },
      });
      final c = _boot(http, completed: done, active: 'customer');
      final t = c.read(tutorialProvider.notifier);
      await t.ready;
      await t.resume();
      await pumpEventQueue(times: 100);
      final s = c.read(tutorialProvider);
      expect(s.values['ticketId'], 't1');
      expect(s.values['ticketNumber'], 'T2');
      expect(s.stepIndex, 2, reason: 'zone and ticket exist: the helpdesk list is next');
      t.routeChanged('/helpdesk/t1');
      t.routeChanged('/helpdesk');
      expect(c.read(tutorialProvider).stepIndex, 3);
    });
  });

  group('chapter 8: the quotation', () {
    final done = ['welcome', 'product', 'suppliers', 'buying', 'invoices', 'manufacturing', 'prices', 'customer'];
    test('create, note, send, pdf', () async {
      final http = FakeHttp({
        'POST /api/v1/quotations': (b) => {
          'id': 'q1',
          'customer_id': (b as Map)['customer_id'],
          'lines': b['lines'],
        },
        'PUT /api/v1/quotations/q1': (b) => {'id': 'q1', 'notes': (b as Map)['notes']},
        'POST /api/v1/quotations/q1/send': (_) => {'id': 'q1', 'status': 'sent'},
      });
      final c = _boot(http, completed: done, active: 'quotation', values: {'customerId': 'cu1'});
      final t = c.read(tutorialProvider.notifier);
      await t.ready;
      await t.resume();
      final dio = c.read(dioProvider);
      int step() => c.read(tutorialProvider).stepIndex;
      t.next();
      t.routeChanged('/quotations');
      expect(step(), 2);
      expect(
        c.read(tutorialProvider).context.resolve(allMissions[8].steps[2].route!),
        '/quotations/new?customer=cu1',
      );
      await dio.post<Object?>('/api/v1/quotations', data: {
        'customer_id': 'other',
        'lines': [{'product_id': 'k1', 'quantity': '8'}],
      });
      expect(step(), 2);
      await dio.post<Object?>('/api/v1/quotations', data: {
        'customer_id': 'cu1',
        'lines': [{'product_id': 'zzz', 'quantity': '8'}],
      });
      expect(step(), 2, reason: 'the cart is needed');
      await dio.post<Object?>('/api/v1/quotations', data: {
        'customer_id': 'cu1',
        'lines': [{'product_id': 'k1', 'quantity': '8'}],
      });
      expect(step(), 3);
      await dio.put<Object?>('/api/v1/quotations/q1', data: {'notes': ' '});
      expect(step(), 3, reason: 'a note is needed');
      await dio.put<Object?>('/api/v1/quotations/q1', data: {'notes': 'About 3 weeks'});
      expect(step(), 4);
      await dio.post<Object?>('/api/v1/quotations/q1/send');
      expect(step(), 5);
      t.next(); // pdf
      t.next();
      expect(c.read(tutorialProvider).completed, contains('quotation'));
    });

    test('probes skip a sent quotation', () async {
      final http = FakeHttp({
        'GET /api/v1/customers': (_) => {
          'items': [{'id': 'cu1', 'name': 'Brightwheel Logistics BV'}],
        },
        'GET /api/v1/quotations': (_) => {
          'items': [{'id': 'q1', 'customer_id': 'cu1'}],
        },
        'GET /api/v1/quotations/q1': (_) => {'status': 'sent', 'notes': 'x'},
      });
      final c = _boot(http, completed: done, active: 'quotation');
      final t = c.read(tutorialProvider.notifier);
      await t.ready;
      await t.resume();
      await pumpEventQueue(times: 100);
      t.routeChanged('/quotations');
      await pumpEventQueue(times: 200);
      expect(
        http.queries.any((q) => q['customer_id'] == 'cu1'),
        isTrue,
        reason: '{customerId} is resolved in the probe query',
      );
      expect(c.read(tutorialProvider).values['quotationId'], 'q1');
      expect(c.read(tutorialProvider).stepIndex, 5, reason: 'only the PDF look is left');
    });
  });

  group('chapter 9: change, order, production', () {
    final done = ['welcome', 'product', 'suppliers', 'buying', 'invoices', 'manufacturing', 'prices', 'customer', 'quotation'];
    test('revision to production, then stock', () async {
      final http = FakeHttp({
        'POST /api/v1/helpdesk/inbound': (_) => {
          'ticket_id': 't1',
          'ticket_number': 'T1',
          'new_ticket': false,
          'duplicate': false,
        },
        'POST /api/v1/quotations/q1/revise': (_) => {'id': 'q2'},
        'PUT /api/v1/quotations/q2': (b) => {
          'id': 'q2',
          'lines': (b as Map)['lines'],
        },
        'POST /api/v1/quotations/q2/send': (_) => {'id': 'q2'},
        'POST /api/v1/quotations/q2/accept': (_) => {'id': 'q2'},
        'POST /api/v1/quotations/q2/convert': (_) => {'order': {'id': 'o1'}},
        'POST /api/v1/orders/o1/send': (_) => {'order': {'id': 'o1'}},
        'POST /api/v1/orders/o1/confirm': (_) => {'order': {'id': 'o1'}},
        'POST /api/v1/production-orders': (b) {
          final m = b as Map;
          return {
            'id': m['order_id'] == null ? 'ps1' : 'pr1',
            'product_id': m['product_id'] ?? 'k1',
            'order_id': m['order_id'],
          };
        },
        'POST /api/v1/production-orders/pr1/start': (_) => {'id': 'pr1'},
        'POST /api/v1/production-orders/pr1/complete': (_) => {'id': 'pr1'},
      });
      final c = _boot(
        http,
        completed: done,
        active: 'change',
        values: {
          'customerId': 'cu1',
          'quotationId': 'q1',
          'mailId1': '<m1@example.test>',
          'ticketNumber': 'T1',
          'productionId': 'p5',
        },
      );
      final t = c.read(tutorialProvider.notifier);
      await t.ready;
      await t.resume();
      final dio = c.read(dioProvider);
      int step() => c.read(tutorialProvider).stepIndex;
      t.next(); // needs
      await t.runAction();
      expect(step(), 2);
      expect(
        http.bodies['POST /api/v1/helpdesk/inbound']! as String,
        contains('In-Reply-To: <m1@example.test>'),
      );
      t.routeChanged('/quotations/q1');
      expect(step(), 3);
      await dio.post<Object?>('/api/v1/quotations/q1/revise', data: {'pricing': 'keep'});
      expect(step(), 4);
      expect(c.read(tutorialProvider).values['quotation2Id'], 'q2');
      Future<void> edit(String qty) => dio.put<Object?>('/api/v1/quotations/q2', data: {
        'lines': [{'product_id': 'k1', 'quantity': qty}],
      });
      await edit('8');
      expect(step(), 4, reason: 'still 8');
      await edit('10');
      expect(step(), 5);
      await dio.post<Object?>('/api/v1/quotations/q2/send');
      await dio.post<Object?>('/api/v1/quotations/q2/accept');
      expect(step(), 7);
      await dio.post<Object?>('/api/v1/quotations/q2/convert', data: {});
      expect(step(), 8);
      expect(c.read(tutorialProvider).values['orderId'], 'o1');
      t.routeChanged('/orders/o1');
      await dio.post<Object?>('/api/v1/orders/o1/send');
      await dio.post<Object?>('/api/v1/orders/o1/confirm', data: {'revision': 1});
      expect(step(), 11);
      await dio.post<Object?>('/api/v1/production-orders', data: {'order_id': 'o1', 'order_line_id': 'l1'});
      expect(step(), 12);
      expect(c.read(tutorialProvider).values['orderProductionId'], 'pr1');
      await dio.post<Object?>('/api/v1/production-orders/pr1/start');
      await dio.post<Object?>('/api/v1/production-orders/pr1/complete');
      expect(step(), 14);
      t.next(); // ready
      expect(step(), 15);
      await dio.post<Object?>('/api/v1/production-orders', data: {'product_id': 'k1', 'quantity': '4'});
      expect(step(), 16);
      t.next();
      expect(c.read(tutorialProvider).completed, contains('change'));
    });
  });
}
