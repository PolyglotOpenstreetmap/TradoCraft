// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'new_model_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

NewModelInput _$NewModelInputFromJson(Map<String, dynamic> json) =>
    NewModelInput(
      code: json['code'] as String,
      name: json['name'] as String,
      definition: json['definition'],
      description: json['description'] as String?,
      productId: json['product_id'] as String?,
      translations: json['translations'],
    );

Map<String, dynamic> _$NewModelInputToJson(NewModelInput instance) =>
    <String, dynamic>{
      'code': instance.code,
      'definition': ?instance.definition,
      'description': ?instance.description,
      'name': instance.name,
      'product_id': ?instance.productId,
      'translations': ?instance.translations,
    };
