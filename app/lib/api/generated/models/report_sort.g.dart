// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'report_sort.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ReportSort _$ReportSortFromJson(Map<String, dynamic> json) => ReportSort(
  column: json['column'] as String,
  dir: json['dir'] == null ? null : ReportDir.fromJson(json['dir'] as String),
);

Map<String, dynamic> _$ReportSortToJson(ReportSort instance) =>
    <String, dynamic>{'column': instance.column, 'dir': ?instance.dir};
