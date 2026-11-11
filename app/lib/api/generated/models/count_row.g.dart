// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'count_row.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

CountRow _$CountRowFromJson(Map<String, dynamic> json) => CountRow(
  count: (json['count'] as num).toInt(),
  key: json['key'] as String,
  label: json['label'] as String,
);

Map<String, dynamic> _$CountRowToJson(CountRow instance) => <String, dynamic>{
  'count': instance.count,
  'key': instance.key,
  'label': instance.label,
};
