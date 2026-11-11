// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'packaging_category.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

PackagingCategory _$PackagingCategoryFromJson(Map<String, dynamic> json) =>
    PackagingCategory(
      active: json['active'] as bool,
      code: json['code'] as String,
      description: json['description'] as String,
      id: json['id'] as String,
      name: json['name'] as String,
      productCount: (json['product_count'] as num).toInt(),
      requiresShippingQuote: json['requires_shipping_quote'] as bool,
    );

Map<String, dynamic> _$PackagingCategoryToJson(PackagingCategory instance) =>
    <String, dynamic>{
      'active': instance.active,
      'code': instance.code,
      'description': instance.description,
      'id': instance.id,
      'name': instance.name,
      'product_count': instance.productCount,
      'requires_shipping_quote': instance.requiresShippingQuote,
    };
