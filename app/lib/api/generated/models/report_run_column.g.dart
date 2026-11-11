// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'report_run_column.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ReportRunColumn _$ReportRunColumnFromJson(Map<String, dynamic> json) =>
    ReportRunColumn(
      key: json['key'] as String,
      kind: ReportColumnKind.fromJson(json['kind'] as String),
      label: json['label'] as String,
      type: ReportFieldType.fromJson(json['type'] as String),
    );

Map<String, dynamic> _$ReportRunColumnToJson(ReportRunColumn instance) =>
    <String, dynamic>{
      'key': instance.key,
      'kind': instance.kind,
      'label': instance.label,
      'type': instance.type,
    };
