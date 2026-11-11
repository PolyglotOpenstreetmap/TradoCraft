// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'suggestion_out.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

SuggestionOut _$SuggestionOutFromJson(Map<String, dynamic> json) =>
    SuggestionOut(
      missingSkillIds: (json['missing_skill_ids'] as List<dynamic>)
          .map((e) => e as String)
          .toList(),
      scheduledEnd: DateTime.parse(json['scheduled_end'] as String),
      scheduledStart: DateTime.parse(json['scheduled_start'] as String),
      slackMinutes: (json['slack_minutes'] as num).toInt(),
      technicianId: json['technician_id'] as String,
      technicianName: json['technician_name'] as String,
    );

Map<String, dynamic> _$SuggestionOutToJson(SuggestionOut instance) =>
    <String, dynamic>{
      'missing_skill_ids': instance.missingSkillIds,
      'scheduled_end': instance.scheduledEnd.toIso8601String(),
      'scheduled_start': instance.scheduledStart.toIso8601String(),
      'slack_minutes': instance.slackMinutes,
      'technician_id': instance.technicianId,
      'technician_name': instance.technicianName,
    };
