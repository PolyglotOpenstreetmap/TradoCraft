// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'explosion_node.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ExplosionNode _$ExplosionNodeFromJson(Map<String, dynamic> json) =>
    ExplosionNode(
      code: json['code'] as String,
      isLeaf: json['is_leaf'] as bool,
      level: (json['level'] as num).toInt(),
      name: json['name'] as String,
      productId: json['product_id'] as String,
      productType: json['product_type'] as String,
      quantityPerParent: json['quantity_per_parent'] as String,
      requiredQuantity: json['required_quantity'] as String,
      scrapPercent: json['scrap_percent'] as String,
      unit: json['unit'] as String,
      parentIndex: (json['parent_index'] as num?)?.toInt(),
    );

Map<String, dynamic> _$ExplosionNodeToJson(ExplosionNode instance) =>
    <String, dynamic>{
      'code': instance.code,
      'is_leaf': instance.isLeaf,
      'level': instance.level,
      'name': instance.name,
      'parent_index': ?instance.parentIndex,
      'product_id': instance.productId,
      'product_type': instance.productType,
      'quantity_per_parent': instance.quantityPerParent,
      'required_quantity': instance.requiredQuantity,
      'scrap_percent': instance.scrapPercent,
      'unit': instance.unit,
    };
