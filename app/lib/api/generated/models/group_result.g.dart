// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'group_result.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

GroupResult _$GroupResultFromJson(Map<String, dynamic> json) => GroupResult(
  name: json['name'] as String,
  values: (json['values'] as List<dynamic>)
      .map((e) => ValueResult.fromJson(e as Map<String, dynamic>))
      .toList(),
);

Map<String, dynamic> _$GroupResultToJson(GroupResult instance) =>
    <String, dynamic>{'name': instance.name, 'values': instance.values};
