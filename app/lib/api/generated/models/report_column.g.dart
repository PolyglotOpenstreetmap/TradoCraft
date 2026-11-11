// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'report_column.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ReportColumn _$ReportColumnFromJson(Map<String, dynamic> json) => ReportColumn(
  field: json['field'] as String,
  label: json['label'] as String?,
  summary: json['summary'] == null
      ? null
      : ReportSummary.fromJson(json['summary'] as String),
);

Map<String, dynamic> _$ReportColumnToJson(ReportColumn instance) =>
    <String, dynamic>{
      'field': instance.field,
      'label': ?instance.label,
      'summary': ?instance.summary,
    };
