// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'report_definition.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ReportDefinition _$ReportDefinitionFromJson(Map<String, dynamic> json) =>
    ReportDefinition(
      columns: (json['columns'] as List<dynamic>)
          .map((e) => ReportColumn.fromJson(e as Map<String, dynamic>))
          .toList(),
      object: json['object'] as String,
      chart: json['chart'] == null
          ? null
          : ReportChart.fromJson(json['chart'] as Map<String, dynamic>),
      filterLogic: json['filter_logic'] == null
          ? null
          : ReportLogic.fromJson(json['filter_logic'] as String),
      filters: (json['filters'] as List<dynamic>?)
          ?.map((e) => ReportFilter.fromJson(e as Map<String, dynamic>))
          .toList(),
      groupBy: (json['group_by'] as List<dynamic>?)
          ?.map((e) => ReportGroupBy.fromJson(e as Map<String, dynamic>))
          .toList(),
      joins: (json['joins'] as List<dynamic>?)
          ?.map((e) => e as String)
          .toList(),
      sort: (json['sort'] as List<dynamic>?)
          ?.map((e) => ReportSort.fromJson(e as Map<String, dynamic>))
          .toList(),
    );

Map<String, dynamic> _$ReportDefinitionToJson(ReportDefinition instance) =>
    <String, dynamic>{
      'chart': ?instance.chart,
      'columns': instance.columns,
      'filter_logic': ?instance.filterLogic,
      'filters': ?instance.filters,
      'group_by': ?instance.groupBy,
      'joins': ?instance.joins,
      'object': instance.object,
      'sort': ?instance.sort,
    };
