// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'business_calendar.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

BusinessCalendar _$BusinessCalendarFromJson(Map<String, dynamic> json) =>
    BusinessCalendar(
      enabled: json['enabled'] as bool,
      holidays: (json['holidays'] as List<dynamic>)
          .map((e) => Holiday.fromJson(e as Map<String, dynamic>))
          .toList(),
      holidaysInherited: json['holidays_inherited'] as bool,
      hours: (json['hours'] as List<dynamic>)
          .map((e) => WorkingHours.fromJson(e as Map<String, dynamic>))
          .toList(),
      inherited: json['inherited'] as bool,
      timeZone: json['time_zone'] as String,
      teamId: json['team_id'] as String?,
    );

Map<String, dynamic> _$BusinessCalendarToJson(BusinessCalendar instance) =>
    <String, dynamic>{
      'enabled': instance.enabled,
      'holidays': instance.holidays,
      'holidays_inherited': instance.holidaysInherited,
      'hours': instance.hours,
      'inherited': instance.inherited,
      'team_id': ?instance.teamId,
      'time_zone': instance.timeZone,
    };
