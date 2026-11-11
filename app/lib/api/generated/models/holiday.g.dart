// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'holiday.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Holiday _$HolidayFromJson(Map<String, dynamic> json) => Holiday(
  day: DateTime.parse(json['day'] as String),
  id: json['id'] as String,
  name: json['name'] as String,
);

Map<String, dynamic> _$HolidayToJson(Holiday instance) => <String, dynamic>{
  'day': instance.day.toIso8601String(),
  'id': instance.id,
  'name': instance.name,
};
