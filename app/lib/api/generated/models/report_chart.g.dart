// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'report_chart.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ReportChart _$ReportChartFromJson(Map<String, dynamic> json) => ReportChart(
  kind: ReportChartKind.fromJson(json['kind'] as String),
  x: json['x'] as String,
  y: json['y'] as String,
);

Map<String, dynamic> _$ReportChartToJson(ReportChart instance) =>
    <String, dynamic>{'kind': instance.kind, 'x': instance.x, 'y': instance.y};
