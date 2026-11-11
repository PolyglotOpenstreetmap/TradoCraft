// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'model.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Model _$ModelFromJson(Map<String, dynamic> json) => Model(
  active: json['active'] as bool,
  code: json['code'] as String,
  createdAt: DateTime.parse(json['created_at'] as String),
  description: json['description'] as String,
  id: json['id'] as String,
  name: json['name'] as String,
  revisions: (json['revisions'] as List<dynamic>)
      .map((e) => RevisionSummary.fromJson(e as Map<String, dynamic>))
      .toList(),
  translations: json['translations'],
  updatedAt: DateTime.parse(json['updated_at'] as String),
  productId: json['product_id'] as String?,
);

Map<String, dynamic> _$ModelToJson(Model instance) => <String, dynamic>{
  'active': instance.active,
  'code': instance.code,
  'created_at': instance.createdAt.toIso8601String(),
  'description': instance.description,
  'id': instance.id,
  'name': instance.name,
  'product_id': ?instance.productId,
  'revisions': instance.revisions,
  'translations': instance.translations,
  'updated_at': instance.updatedAt.toIso8601String(),
};
