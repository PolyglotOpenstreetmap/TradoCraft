// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'report_catalog.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ReportCatalog _$ReportCatalogFromJson(Map<String, dynamic> json) =>
    ReportCatalog(
      links: (json['links'] as List<dynamic>)
          .map((e) => ReportCatalogLink.fromJson(e as Map<String, dynamic>))
          .toList(),
      objects: (json['objects'] as List<dynamic>)
          .map((e) => ReportCatalogObject.fromJson(e as Map<String, dynamic>))
          .toList(),
    );

Map<String, dynamic> _$ReportCatalogToJson(ReportCatalog instance) =>
    <String, dynamic>{'links': instance.links, 'objects': instance.objects};
