// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'repair_event.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

RepairEvent _$RepairEventFromJson(Map<String, dynamic> json) => RepairEvent(
  actorName: json['actor_name'] as String,
  createdAt: DateTime.parse(json['created_at'] as String),
  id: json['id'] as String,
  kind: json['kind'] as String,
  note: json['note'] as String,
  fromDepartment: json['from_department'] as String?,
  fromStatus: json['from_status'] as String?,
  toDepartment: json['to_department'] as String?,
  toStatus: json['to_status'] as String?,
);

Map<String, dynamic> _$RepairEventToJson(RepairEvent instance) =>
    <String, dynamic>{
      'actor_name': instance.actorName,
      'created_at': instance.createdAt.toIso8601String(),
      'from_department': ?instance.fromDepartment,
      'from_status': ?instance.fromStatus,
      'id': instance.id,
      'kind': instance.kind,
      'note': instance.note,
      'to_department': ?instance.toDepartment,
      'to_status': ?instance.toStatus,
    };
