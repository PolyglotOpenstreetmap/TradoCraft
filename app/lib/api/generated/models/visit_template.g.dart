// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'visit_template.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

VisitTemplate _$VisitTemplateFromJson(Map<String, dynamic> json) =>
    VisitTemplate(
      active: json['active'] as bool,
      anchorDate: DateTime.parse(json['anchor_date'] as String),
      id: json['id'] as String,
      instructions: json['instructions'] as String,
      intervalN: (json['interval_n'] as num).toInt(),
      kind: json['kind'] as String,
      leadDays: (json['lead_days'] as num).toInt(),
      months: (json['months'] as List<dynamic>)
          .map((e) => (e as num).toInt())
          .toList(),
      position: (json['position'] as num).toInt(),
      recurrence: json['recurrence'] as String,
      tasks: (json['tasks'] as List<dynamic>).map((e) => e as String).toList(),
      technicians: (json['technicians'] as List<dynamic>)
          .map((e) => Technician.fromJson(e as Map<String, dynamic>))
          .toList(),
      title: json['title'] as String,
      estimatedMinutes: (json['estimated_minutes'] as num?)?.toInt(),
      nextDue: json['next_due'] == null
          ? null
          : DateTime.parse(json['next_due'] as String),
    );

Map<String, dynamic> _$VisitTemplateToJson(VisitTemplate instance) =>
    <String, dynamic>{
      'active': instance.active,
      'anchor_date': instance.anchorDate.toIso8601String(),
      'estimated_minutes': ?instance.estimatedMinutes,
      'id': instance.id,
      'instructions': instance.instructions,
      'interval_n': instance.intervalN,
      'kind': instance.kind,
      'lead_days': instance.leadDays,
      'months': instance.months,
      'next_due': ?instance.nextDue?.toIso8601String(),
      'position': instance.position,
      'recurrence': instance.recurrence,
      'tasks': instance.tasks,
      'technicians': instance.technicians,
      'title': instance.title,
    };
