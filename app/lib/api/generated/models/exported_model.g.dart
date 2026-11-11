// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'exported_model.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ExportedModel _$ExportedModelFromJson(Map<String, dynamic> json) =>
    ExportedModel(
      code: json['code'] as String,
      name: json['name'] as String,
      description: json['description'] as String?,
      translations: json['translations'],
    );

Map<String, dynamic> _$ExportedModelToJson(ExportedModel instance) =>
    <String, dynamic>{
      'code': instance.code,
      'description': ?instance.description,
      'name': instance.name,
      'translations': ?instance.translations,
    };
