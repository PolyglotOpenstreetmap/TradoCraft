// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'product_family.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ProductFamily _$ProductFamilyFromJson(Map<String, dynamic> json) =>
    ProductFamily(
      active: json['active'] as bool,
      code: json['code'] as String,
      id: json['id'] as String,
      name: json['name'] as String,
      productCount: (json['product_count'] as num).toInt(),
    );

Map<String, dynamic> _$ProductFamilyToJson(ProductFamily instance) =>
    <String, dynamic>{
      'active': instance.active,
      'code': instance.code,
      'id': instance.id,
      'name': instance.name,
      'product_count': instance.productCount,
    };
