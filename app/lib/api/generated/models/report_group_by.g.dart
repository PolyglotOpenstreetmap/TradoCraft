// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'report_group_by.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ReportGroupBy _$ReportGroupByFromJson(Map<String, dynamic> json) =>
    ReportGroupBy(
      field: json['field'] as String,
      bucket: json['bucket'] == null
          ? null
          : ReportBucket.fromJson(json['bucket'] as String),
    );

Map<String, dynamic> _$ReportGroupByToJson(ReportGroupBy instance) =>
    <String, dynamic>{'bucket': ?instance.bucket, 'field': instance.field};
