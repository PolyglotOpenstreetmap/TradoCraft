// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'item_table_result.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ItemTableResult _$ItemTableResultFromJson(Map<String, dynamic> json) =>
    ItemTableResult(
      rows: (json['rows'] as List<dynamic>)
          .map((e) => ItemResult.fromJson(e as Map<String, dynamic>))
          .toList(),
      table: json['table'] as String,
    );

Map<String, dynamic> _$ItemTableResultToJson(ItemTableResult instance) =>
    <String, dynamic>{'rows': instance.rows, 'table': instance.table};
