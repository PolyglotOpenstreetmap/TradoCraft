// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'calendar_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

CalendarInput _$CalendarInputFromJson(Map<String, dynamic> json) =>
    CalendarInput(
      enabled: json['enabled'] as bool,
      hours: (json['hours'] as List<dynamic>)
          .map((e) => WorkingHours.fromJson(e as Map<String, dynamic>))
          .toList(),
      timeZone: json['time_zone'] as String,
    );

Map<String, dynamic> _$CalendarInputToJson(CalendarInput instance) =>
    <String, dynamic>{
      'enabled': instance.enabled,
      'hours': instance.hours,
      'time_zone': instance.timeZone,
    };
