// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'schedule_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ScheduleInput _$ScheduleInputFromJson(Map<String, dynamic> json) =>
    ScheduleInput(
      scheduledEnd: DateTime.parse(json['scheduled_end'] as String),
      scheduledStart: DateTime.parse(json['scheduled_start'] as String),
      technicianIds: (json['technician_ids'] as List<dynamic>)
          .map((e) => e as String)
          .toList(),
    );

Map<String, dynamic> _$ScheduleInputToJson(ScheduleInput instance) =>
    <String, dynamic>{
      'scheduled_end': instance.scheduledEnd.toIso8601String(),
      'scheduled_start': instance.scheduledStart.toIso8601String(),
      'technician_ids': instance.technicianIds,
    };
