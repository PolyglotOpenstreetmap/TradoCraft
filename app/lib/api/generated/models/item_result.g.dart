// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'item_result.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ItemResult _$ItemResultFromJson(Map<String, dynamic> json) => ItemResult(
  group: json['group'] as String,
  index: (json['index'] as num).toInt(),
  label: json['label'] as String,
  values: (json['values'] as List<dynamic>)
      .map((e) => ValueResult.fromJson(e as Map<String, dynamic>))
      .toList(),
);

Map<String, dynamic> _$ItemResultToJson(ItemResult instance) =>
    <String, dynamic>{
      'group': instance.group,
      'index': instance.index,
      'label': instance.label,
      'values': instance.values,
    };
