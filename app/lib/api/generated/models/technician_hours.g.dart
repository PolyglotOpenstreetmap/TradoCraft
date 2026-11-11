// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'technician_hours.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

TechnicianHours _$TechnicianHoursFromJson(Map<String, dynamic> json) =>
    TechnicianHours(
      endTime: json['end_time'] as String,
      id: json['id'] as String,
      startTime: json['start_time'] as String,
      weekday: (json['weekday'] as num).toInt(),
      userId: json['user_id'] as String?,
    );

Map<String, dynamic> _$TechnicianHoursToJson(TechnicianHours instance) =>
    <String, dynamic>{
      'end_time': instance.endTime,
      'id': instance.id,
      'start_time': instance.startTime,
      'user_id': ?instance.userId,
      'weekday': instance.weekday,
    };
