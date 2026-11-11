// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'report_run_result.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ReportRunResult _$ReportRunResultFromJson(Map<String, dynamic> json) =>
    ReportRunResult(
      asOf: DateTime.parse(json['as_of'] as String),
      columns: (json['columns'] as List<dynamic>)
          .map((e) => ReportRunColumn.fromJson(e as Map<String, dynamic>))
          .toList(),
      rows: (json['rows'] as List<dynamic>)
          .map((e) => e as List<dynamic>)
          .toList(),
      total: (json['total'] as num).toInt(),
      chart: json['chart'] == null
          ? null
          : ReportChart.fromJson(json['chart'] as Map<String, dynamic>),
      totals: json['totals'] as List<dynamic>?,
    );

Map<String, dynamic> _$ReportRunResultToJson(ReportRunResult instance) =>
    <String, dynamic>{
      'as_of': instance.asOf.toIso8601String(),
      'chart': ?instance.chart,
      'columns': instance.columns,
      'rows': instance.rows,
      'total': instance.total,
      'totals': ?instance.totals,
    };
