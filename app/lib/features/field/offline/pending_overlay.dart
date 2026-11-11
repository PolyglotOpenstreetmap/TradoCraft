import 'dart:convert';

import 'offline_queue.dart';
import '../../../app/api.dart';

/// The ids of the records on screen that exist only on this device (or whose change is waiting).
Set<String> pendingIds(Iterable<QueuedOp> ops) => {
  for (final o in ops) o.entityId ?? o.id,
};

/// The job as the technician has left it: what the server last said, with the queued changes
/// applied on top, in order. Used to show the work at once while offline.
JobDetail applyPending(
  JobDetail server,
  Iterable<QueuedOp> ops, {
  String userName = '',
}) {
  final list = ops.toList();
  if (list.isEmpty) return server;
  // `jsonEncode` calls `toJson` on the nested models.
  final m = jsonDecode(jsonEncode(server)) as Map<String, dynamic>;
  final job = m['job'] as Map<String, dynamic>;
  final tasks = (m['tasks'] as List).cast<Map<String, dynamic>>();
  final logs = (m['work_logs'] as List).cast<Map<String, dynamic>>();
  final parts = (m['parts'] as List).cast<Map<String, dynamic>>();

  for (final o in list) {
    final p = o.payload;
    switch (o.type) {
      case OpType.start:
        if (job['status'] == 'planned') job['status'] = 'in_progress';
      case OpType.taskDone:
        for (final t in tasks.where((t) => t['id'] == o.entityId)) {
          t['done'] = p['done'] == true;
        }
      case OpType.taskAdd:
        tasks.add({
          'id': o.id,
          'position': tasks.length + 1,
          'description': p['description'],
          'done': false,
        });
      case OpType.workLog:
        final from = DateTime.parse('${p['started_at']}');
        final to = DateTime.parse('${p['ended_at']}');
        final minutes = to.difference(from).inMinutes;
        logs.add({
          'id': o.id,
          'user_id': '',
          'user_name': userName,
          'kind': p['kind'],
          'started_at': p['started_at'],
          'ended_at': p['ended_at'],
          'minutes': minutes,
          'notes': p['notes'] ?? '',
        });
        final key = p['kind'] == 'travel' ? 'travel_minutes' : 'work_minutes';
        m[key] = (m[key] as num) + minutes;
      case OpType.partAdd:
        parts.add({
          'id': o.id,
          'product_id': p['product_id'],
          'description': p['description'],
          'quantity': p['quantity'],
          'unit': p['unit'] ?? 'pcs',
          'billable': p['billable'] ?? true,
          'added_by_name': userName,
          'created_at': o.createdAt.toIso8601String(),
        });
      case OpType.complete:
        job['status'] = 'completed';
        job['completion_notes'] = p['notes'];
      case OpType.decline:
        m['signature'] = {
          'signer_name': p['signer_name'] ?? '',
          'declined': true,
          'decline_reason': p['reason'],
          'has_image': false,
          'signed_at': o.createdAt.toIso8601String(),
        };
      case OpType.signature:
        m['signature'] = {
          'signer_name': p['signer_name'] ?? '',
          'declined': false,
          'decline_reason': '',
          'has_image': false,
          'signed_at': o.createdAt.toIso8601String(),
        };
      case OpType.photo:
        break; // shown as a count: the picture is not on the server yet
    }
  }
  job['tasks_total'] = tasks.length;
  job['tasks_done'] = tasks.where((t) => t['done'] == true).length;
  return JobDetail.fromJson(m);
}
