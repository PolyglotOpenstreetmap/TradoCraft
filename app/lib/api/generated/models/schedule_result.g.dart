// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'schedule_result.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ScheduleResult _$ScheduleResultFromJson(Map<String, dynamic> json) =>
    ScheduleResult(
      applied: json['applied'] as bool,
      conflicts: (json['conflicts'] as List<dynamic>)
          .map((e) => Conflict.fromJson(e as Map<String, dynamic>))
          .toList(),
      job: json['job'] == null
          ? null
          : BoardJob.fromJson(json['job'] as Map<String, dynamic>),
    );

Map<String, dynamic> _$ScheduleResultToJson(ScheduleResult instance) =>
    <String, dynamic>{
      'applied': instance.applied,
      'conflicts': instance.conflicts,
      'job': ?instance.job,
    };
