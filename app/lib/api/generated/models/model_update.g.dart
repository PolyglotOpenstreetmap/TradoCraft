// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'model_update.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ModelUpdate _$ModelUpdateFromJson(Map<String, dynamic> json) => ModelUpdate(
  active: json['active'] as bool,
  name: json['name'] as String,
  description: json['description'] as String?,
  productId: json['product_id'] as String?,
  translations: json['translations'],
);

Map<String, dynamic> _$ModelUpdateToJson(ModelUpdate instance) =>
    <String, dynamic>{
      'active': instance.active,
      'description': ?instance.description,
      'name': instance.name,
      'product_id': ?instance.productId,
      'translations': ?instance.translations,
    };
