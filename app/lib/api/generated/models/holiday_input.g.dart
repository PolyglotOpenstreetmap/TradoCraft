// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'holiday_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

HolidayInput _$HolidayInputFromJson(Map<String, dynamic> json) => HolidayInput(
  day: DateTime.parse(json['day'] as String),
  name: json['name'] as String,
);

Map<String, dynamic> _$HolidayInputToJson(HolidayInput instance) =>
    <String, dynamic>{
      'day': instance.day.toIso8601String(),
      'name': instance.name,
    };
