// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'report_catalog_object.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ReportCatalogObject _$ReportCatalogObjectFromJson(Map<String, dynamic> json) =>
    ReportCatalogObject(
      fields: (json['fields'] as List<dynamic>)
          .map((e) => ReportCatalogField.fromJson(e as Map<String, dynamic>))
          .toList(),
      label: json['label'] as String,
      name: json['name'] as String,
    );

Map<String, dynamic> _$ReportCatalogObjectToJson(
  ReportCatalogObject instance,
) => <String, dynamic>{
  'fields': instance.fields,
  'label': instance.label,
  'name': instance.name,
};
