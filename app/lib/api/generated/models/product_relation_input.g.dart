// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'product_relation_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ProductRelationInput _$ProductRelationInputFromJson(
  Map<String, dynamic> json,
) => ProductRelationInput(
  kind: json['kind'] as String,
  aFamilyId: json['a_family_id'] as String?,
  aProductId: json['a_product_id'] as String?,
  active: json['active'] as bool?,
  bFamilyId: json['b_family_id'] as String?,
  bProductId: json['b_product_id'] as String?,
  bidirectional: json['bidirectional'] as bool?,
  note: json['note'] as String?,
  strength: (json['strength'] as num?)?.toInt(),
);

Map<String, dynamic> _$ProductRelationInputToJson(
  ProductRelationInput instance,
) => <String, dynamic>{
  'a_family_id': ?instance.aFamilyId,
  'a_product_id': ?instance.aProductId,
  'active': ?instance.active,
  'b_family_id': ?instance.bFamilyId,
  'b_product_id': ?instance.bProductId,
  'bidirectional': ?instance.bidirectional,
  'kind': instance.kind,
  'note': ?instance.note,
  'strength': ?instance.strength,
};
