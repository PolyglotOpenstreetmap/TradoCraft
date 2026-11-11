// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'product_relation.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ProductRelation _$ProductRelationFromJson(Map<String, dynamic> json) =>
    ProductRelation(
      active: json['active'] as bool,
      bidirectional: json['bidirectional'] as bool,
      id: json['id'] as String,
      kind: json['kind'] as String,
      note: json['note'] as String,
      strength: (json['strength'] as num).toInt(),
      aFamilyCode: json['a_family_code'] as String?,
      aFamilyId: json['a_family_id'] as String?,
      aProductCode: json['a_product_code'] as String?,
      aProductId: json['a_product_id'] as String?,
      bFamilyCode: json['b_family_code'] as String?,
      bFamilyId: json['b_family_id'] as String?,
      bProductCode: json['b_product_code'] as String?,
      bProductId: json['b_product_id'] as String?,
    );

Map<String, dynamic> _$ProductRelationToJson(ProductRelation instance) =>
    <String, dynamic>{
      'a_family_code': ?instance.aFamilyCode,
      'a_family_id': ?instance.aFamilyId,
      'a_product_code': ?instance.aProductCode,
      'a_product_id': ?instance.aProductId,
      'active': instance.active,
      'b_family_code': ?instance.bFamilyCode,
      'b_family_id': ?instance.bFamilyId,
      'b_product_code': ?instance.bProductCode,
      'b_product_id': ?instance.bProductId,
      'bidirectional': instance.bidirectional,
      'id': instance.id,
      'kind': instance.kind,
      'note': instance.note,
      'strength': instance.strength,
    };
