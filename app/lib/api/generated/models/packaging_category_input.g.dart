// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'packaging_category_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

PackagingCategoryInput _$PackagingCategoryInputFromJson(
  Map<String, dynamic> json,
) => PackagingCategoryInput(
  code: json['code'] as String,
  name: json['name'] as String,
  active: json['active'] as bool?,
  description: json['description'] as String?,
  requiresShippingQuote: json['requires_shipping_quote'] as bool?,
);

Map<String, dynamic> _$PackagingCategoryInputToJson(
  PackagingCategoryInput instance,
) => <String, dynamic>{
  'active': ?instance.active,
  'code': instance.code,
  'description': ?instance.description,
  'name': instance.name,
  'requires_shipping_quote': ?instance.requiresShippingQuote,
};
