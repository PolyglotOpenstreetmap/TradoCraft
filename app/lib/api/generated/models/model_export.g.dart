// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'model_export.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ModelExport _$ModelExportFromJson(Map<String, dynamic> json) => ModelExport(
  definition: json['definition'],
  format: json['format'] as String,
  formatVersion: (json['format_version'] as num).toInt(),
  model: ExportedModel.fromJson(json['model'] as Map<String, dynamic>),
);

Map<String, dynamic> _$ModelExportToJson(ModelExport instance) =>
    <String, dynamic>{
      'definition': instance.definition,
      'format': instance.format,
      'format_version': instance.formatVersion,
      'model': instance.model,
    };
