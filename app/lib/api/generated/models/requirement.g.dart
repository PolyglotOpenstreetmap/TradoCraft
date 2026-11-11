// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'requirement.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Requirement _$RequirementFromJson(Map<String, dynamic> json) => Requirement(
  code: json['code'] as String,
  name: json['name'] as String,
  productId: json['product_id'] as String,
  productType: json['product_type'] as String,
  quantity: json['quantity'] as String,
  unit: json['unit'] as String,
);

Map<String, dynamic> _$RequirementToJson(Requirement instance) =>
    <String, dynamic>{
      'code': instance.code,
      'name': instance.name,
      'product_id': instance.productId,
      'product_type': instance.productType,
      'quantity': instance.quantity,
      'unit': instance.unit,
    };
