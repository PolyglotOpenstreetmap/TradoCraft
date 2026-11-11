// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'settings.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Settings _$SettingsFromJson(Map<String, dynamic> json) => Settings(
  defaultJobMinutes: (json['default_job_minutes'] as num).toInt(),
  timezone: json['timezone'] as String,
  travelBufferMinutes: (json['travel_buffer_minutes'] as num).toInt(),
);

Map<String, dynamic> _$SettingsToJson(Settings instance) => <String, dynamic>{
  'default_job_minutes': instance.defaultJobMinutes,
  'timezone': instance.timezone,
  'travel_buffer_minutes': instance.travelBufferMinutes,
};
