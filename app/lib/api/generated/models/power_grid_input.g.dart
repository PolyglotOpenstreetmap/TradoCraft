// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'power_grid_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

PowerGridInput _$PowerGridInputFromJson(Map<String, dynamic> json) =>
    PowerGridInput(
      countryCode: json['country_code'] as String,
      frequencyHz: (json['frequency_hz'] as num).toInt(),
      phases: (json['phases'] as num).toInt(),
      voltageV: (json['voltage_v'] as num).toInt(),
      active: json['active'] as bool?,
      name: json['name'] as String?,
    );

Map<String, dynamic> _$PowerGridInputToJson(PowerGridInput instance) =>
    <String, dynamic>{
      'active': ?instance.active,
      'country_code': instance.countryCode,
      'frequency_hz': instance.frequencyHz,
      'name': ?instance.name,
      'phases': instance.phases,
      'voltage_v': instance.voltageV,
    };
