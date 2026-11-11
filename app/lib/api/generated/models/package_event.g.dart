// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'package_event.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

PackageEvent _$PackageEventFromJson(Map<String, dynamic> json) => PackageEvent(
  actorName: json['actor_name'] as String,
  createdAt: DateTime.parse(json['created_at'] as String),
  id: json['id'] as String,
  kind: json['kind'] as String,
  note: json['note'] as String,
  fromStatus: json['from_status'] as String?,
  repairId: json['repair_id'] as String?,
  repairNumber: json['repair_number'] as String?,
  toStatus: json['to_status'] as String?,
);

Map<String, dynamic> _$PackageEventToJson(PackageEvent instance) =>
    <String, dynamic>{
      'actor_name': instance.actorName,
      'created_at': instance.createdAt.toIso8601String(),
      'from_status': ?instance.fromStatus,
      'id': instance.id,
      'kind': instance.kind,
      'note': instance.note,
      'repair_id': ?instance.repairId,
      'repair_number': ?instance.repairNumber,
      'to_status': ?instance.toStatus,
    };
