// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'work_log_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

WorkLogInput _$WorkLogInputFromJson(Map<String, dynamic> json) => WorkLogInput(
  endedAt: DateTime.parse(json['ended_at'] as String),
  startedAt: DateTime.parse(json['started_at'] as String),
  id: json['id'] as String?,
  kind: json['kind'] as String?,
  notes: json['notes'] as String?,
);

Map<String, dynamic> _$WorkLogInputToJson(WorkLogInput instance) =>
    <String, dynamic>{
      'ended_at': instance.endedAt.toIso8601String(),
      'id': ?instance.id,
      'kind': ?instance.kind,
      'notes': ?instance.notes,
      'started_at': instance.startedAt.toIso8601String(),
    };
