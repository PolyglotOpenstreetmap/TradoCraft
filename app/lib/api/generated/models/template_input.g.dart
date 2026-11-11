// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'template_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

TemplateInput _$TemplateInputFromJson(Map<String, dynamic> json) =>
    TemplateInput(
      recurrence: json['recurrence'] as String,
      title: json['title'] as String,
      active: json['active'] as bool?,
      anchorDate: json['anchor_date'] == null
          ? null
          : DateTime.parse(json['anchor_date'] as String),
      estimatedMinutes: (json['estimated_minutes'] as num?)?.toInt(),
      id: json['id'] as String?,
      instructions: json['instructions'] as String?,
      intervalN: (json['interval_n'] as num?)?.toInt(),
      kind: json['kind'] as String?,
      leadDays: (json['lead_days'] as num?)?.toInt(),
      months: (json['months'] as List<dynamic>?)
          ?.map((e) => (e as num).toInt())
          .toList(),
      tasks: (json['tasks'] as List<dynamic>?)
          ?.map((e) => e as String)
          .toList(),
      technicianIds: (json['technician_ids'] as List<dynamic>?)
          ?.map((e) => e as String)
          .toList(),
    );

Map<String, dynamic> _$TemplateInputToJson(TemplateInput instance) =>
    <String, dynamic>{
      'active': ?instance.active,
      'anchor_date': ?instance.anchorDate?.toIso8601String(),
      'estimated_minutes': ?instance.estimatedMinutes,
      'id': ?instance.id,
      'instructions': ?instance.instructions,
      'interval_n': ?instance.intervalN,
      'kind': ?instance.kind,
      'lead_days': ?instance.leadDays,
      'months': ?instance.months,
      'recurrence': instance.recurrence,
      'tasks': ?instance.tasks,
      'technician_ids': ?instance.technicianIds,
      'title': instance.title,
    };
