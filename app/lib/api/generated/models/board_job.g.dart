// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'board_job.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

BoardJob _$BoardJobFromJson(Map<String, dynamic> json) => BoardJob(
  city: json['city'] as String,
  customerId: json['customer_id'] as String,
  customerName: json['customer_name'] as String,
  id: json['id'] as String,
  kind: json['kind'] as String,
  number: json['number'] as String,
  priority: json['priority'] as String,
  requiredSkillIds: (json['required_skill_ids'] as List<dynamic>)
      .map((e) => e as String)
      .toList(),
  status: json['status'] as String,
  technicianIds: (json['technician_ids'] as List<dynamic>)
      .map((e) => e as String)
      .toList(),
  title: json['title'] as String,
  dueDate: json['due_date'] == null
      ? null
      : DateTime.parse(json['due_date'] as String),
  scheduledEnd: json['scheduled_end'] == null
      ? null
      : DateTime.parse(json['scheduled_end'] as String),
  scheduledStart: json['scheduled_start'] == null
      ? null
      : DateTime.parse(json['scheduled_start'] as String),
);

Map<String, dynamic> _$BoardJobToJson(BoardJob instance) => <String, dynamic>{
  'city': instance.city,
  'customer_id': instance.customerId,
  'customer_name': instance.customerName,
  'due_date': ?instance.dueDate?.toIso8601String(),
  'id': instance.id,
  'kind': instance.kind,
  'number': instance.number,
  'priority': instance.priority,
  'required_skill_ids': instance.requiredSkillIds,
  'scheduled_end': ?instance.scheduledEnd?.toIso8601String(),
  'scheduled_start': ?instance.scheduledStart?.toIso8601String(),
  'status': instance.status,
  'technician_ids': instance.technicianIds,
  'title': instance.title,
};
