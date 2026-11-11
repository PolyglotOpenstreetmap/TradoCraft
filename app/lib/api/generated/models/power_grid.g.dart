// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'power_grid.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

PowerGrid _$PowerGridFromJson(Map<String, dynamic> json) => PowerGrid(
  active: json['active'] as bool,
  countryCode: json['country_code'] as String,
  frequencyHz: (json['frequency_hz'] as num).toInt(),
  id: json['id'] as String,
  name: json['name'] as String,
  phases: (json['phases'] as num).toInt(),
  voltageV: (json['voltage_v'] as num).toInt(),
);

Map<String, dynamic> _$PowerGridToJson(PowerGrid instance) => <String, dynamic>{
  'active': instance.active,
  'country_code': instance.countryCode,
  'frequency_hz': instance.frequencyHz,
  'id': instance.id,
  'name': instance.name,
  'phases': instance.phases,
  'voltage_v': instance.voltageV,
};
