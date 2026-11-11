// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'report_filter.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ReportFilter _$ReportFilterFromJson(Map<String, dynamic> json) => ReportFilter(
  field: json['field'] as String,
  op: ReportOp.fromJson(json['op'] as String),
  value: json['value'],
);

Map<String, dynamic> _$ReportFilterToJson(ReportFilter instance) =>
    <String, dynamic>{
      'field': instance.field,
      'op': instance.op,
      'value': ?instance.value,
    };
