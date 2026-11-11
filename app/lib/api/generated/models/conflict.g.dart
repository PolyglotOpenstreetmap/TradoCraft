// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'conflict.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Conflict _$ConflictFromJson(Map<String, dynamic> json) => Conflict(
  kind: ConflictKind.fromJson(json['kind'] as String),
  messageKey: json['message_key'] as String,
  severity: Severity.fromJson(json['severity'] as String),
  otherJobId: json['other_job_id'] as String?,
  skillId: json['skill_id'] as String?,
  technicianId: json['technician_id'] as String?,
);

Map<String, dynamic> _$ConflictToJson(Conflict instance) => <String, dynamic>{
  'kind': instance.kind,
  'message_key': instance.messageKey,
  'other_job_id': ?instance.otherJobId,
  'severity': instance.severity,
  'skill_id': ?instance.skillId,
  'technician_id': ?instance.technicianId,
};
