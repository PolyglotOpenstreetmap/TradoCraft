// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'work_log.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

WorkLog _$WorkLogFromJson(Map<String, dynamic> json) => WorkLog(
  endedAt: DateTime.parse(json['ended_at'] as String),
  id: json['id'] as String,
  kind: json['kind'] as String,
  minutes: (json['minutes'] as num).toInt(),
  notes: json['notes'] as String,
  startedAt: DateTime.parse(json['started_at'] as String),
  userId: json['user_id'] as String,
  userName: json['user_name'] as String,
);

Map<String, dynamic> _$WorkLogToJson(WorkLog instance) => <String, dynamic>{
  'ended_at': instance.endedAt.toIso8601String(),
  'id': instance.id,
  'kind': instance.kind,
  'minutes': instance.minutes,
  'notes': instance.notes,
  'started_at': instance.startedAt.toIso8601String(),
  'user_id': instance.userId,
  'user_name': instance.userName,
};
