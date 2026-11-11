// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'value_result.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ValueResult _$ValueResultFromJson(Map<String, dynamic> json) => ValueResult(
  enabled: json['enabled'] as bool,
  kind: json['kind'] as String,
  name: json['name'] as String,
  source: ValueSource.fromJson(json['source'] as String),
  value: json['value'],
  visible: json['visible'] as bool,
  calculated: json['calculated'],
  options: json['options'] as List<dynamic>?,
);

Map<String, dynamic> _$ValueResultToJson(ValueResult instance) =>
    <String, dynamic>{
      'calculated': ?instance.calculated,
      'enabled': instance.enabled,
      'kind': instance.kind,
      'name': instance.name,
      'options': ?instance.options,
      'source': instance.source,
      'value': instance.value,
      'visible': instance.visible,
    };
