// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'report_catalog_field.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ReportCatalogField _$ReportCatalogFieldFromJson(Map<String, dynamic> json) =>
    ReportCatalogField(
      countOnly: json['count_only'] as bool,
      label: json['label'] as String,
      name: json['name'] as String,
      type: ReportFieldType.fromJson(json['type'] as String),
    );

Map<String, dynamic> _$ReportCatalogFieldToJson(ReportCatalogField instance) =>
    <String, dynamic>{
      'count_only': instance.countOnly,
      'label': instance.label,
      'name': instance.name,
      'type': instance.type,
    };
