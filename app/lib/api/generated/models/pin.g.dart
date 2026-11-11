// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'pin.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Pin _$PinFromJson(Map<String, dynamic> json) => Pin(
  name: json['name'] as String,
  value: json['value'],
  calculated: json['calculated'],
  group: json['group'] as String?,
  row: (json['row'] as num?)?.toInt(),
  table: json['table'] as String?,
);

Map<String, dynamic> _$PinToJson(Pin instance) => <String, dynamic>{
  'calculated': ?instance.calculated,
  'group': ?instance.group,
  'name': instance.name,
  'row': ?instance.row,
  'table': ?instance.table,
  'value': instance.value,
};
