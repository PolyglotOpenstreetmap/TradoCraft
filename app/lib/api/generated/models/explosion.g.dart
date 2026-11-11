// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'explosion.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Explosion _$ExplosionFromJson(Map<String, dynamic> json) => Explosion(
  bomRevision: (json['bom_revision'] as num).toInt(),
  nodes: (json['nodes'] as List<dynamic>)
      .map((e) => ExplosionNode.fromJson(e as Map<String, dynamic>))
      .toList(),
  productId: json['product_id'] as String,
  quantity: json['quantity'] as String,
  requirements: (json['requirements'] as List<dynamic>)
      .map((e) => Requirement.fromJson(e as Map<String, dynamic>))
      .toList(),
);

Map<String, dynamic> _$ExplosionToJson(Explosion instance) => <String, dynamic>{
  'bom_revision': instance.bomRevision,
  'nodes': instance.nodes,
  'product_id': instance.productId,
  'quantity': instance.quantity,
  'requirements': instance.requirements,
};
