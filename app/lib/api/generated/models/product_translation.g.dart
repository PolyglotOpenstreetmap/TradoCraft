// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'product_translation.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ProductTranslation _$ProductTranslationFromJson(Map<String, dynamic> json) =>
    ProductTranslation(
      language: json['language'] as String,
      name: json['name'] as String,
      description: json['description'] as String?,
    );

Map<String, dynamic> _$ProductTranslationToJson(ProductTranslation instance) =>
    <String, dynamic>{
      'description': ?instance.description,
      'language': instance.language,
      'name': instance.name,
    };
