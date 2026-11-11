// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'working_hours.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

WorkingHours _$WorkingHoursFromJson(Map<String, dynamic> json) => WorkingHours(
  endTime: json['end_time'] as String,
  startTime: json['start_time'] as String,
  weekday: (json['weekday'] as num).toInt(),
);

Map<String, dynamic> _$WorkingHoursToJson(WorkingHours instance) =>
    <String, dynamic>{
      'end_time': instance.endTime,
      'start_time': instance.startTime,
      'weekday': instance.weekday,
    };
