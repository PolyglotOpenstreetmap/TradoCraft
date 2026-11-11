// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'equipment_job.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

EquipmentJob _$EquipmentJobFromJson(Map<String, dynamic> json) => EquipmentJob(
  id: json['id'] as String,
  kind: json['kind'] as String,
  number: json['number'] as String,
  status: json['status'] as String,
  title: json['title'] as String,
  completedAt: json['completed_at'] == null
      ? null
      : DateTime.parse(json['completed_at'] as String),
  scheduledStart: json['scheduled_start'] == null
      ? null
      : DateTime.parse(json['scheduled_start'] as String),
);

Map<String, dynamic> _$EquipmentJobToJson(EquipmentJob instance) =>
    <String, dynamic>{
      'completed_at': ?instance.completedAt?.toIso8601String(),
      'id': instance.id,
      'kind': instance.kind,
      'number': instance.number,
      'scheduled_start': ?instance.scheduledStart?.toIso8601String(),
      'status': instance.status,
      'title': instance.title,
    };
