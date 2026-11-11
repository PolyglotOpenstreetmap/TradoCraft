// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'import_report.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ImportReport _$ImportReportFromJson(Map<String, dynamic> json) => ImportReport(
  created: (json['created'] as num).toInt(),
  errors: (json['errors'] as List<dynamic>)
      .map((e) => RowError.fromJson(e as Map<String, dynamic>))
      .toList(),
  unchanged: (json['unchanged'] as num).toInt(),
  updated: (json['updated'] as num).toInt(),
);

Map<String, dynamic> _$ImportReportToJson(ImportReport instance) =>
    <String, dynamic>{
      'created': instance.created,
      'errors': instance.errors,
      'unchanged': instance.unchanged,
      'updated': instance.updated,
    };
