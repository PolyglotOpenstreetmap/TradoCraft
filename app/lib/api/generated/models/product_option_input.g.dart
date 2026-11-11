// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'product_option_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ProductOptionInput _$ProductOptionInputFromJson(Map<String, dynamic> json) =>
    ProductOptionInput(
      optionProductId: json['option_product_id'] as String,
      active: json['active'] as bool?,
      defaultQuantity: json['default_quantity'] as String?,
      requiredValue: json['required'] as bool?,
      sortOrder: (json['sort_order'] as num?)?.toInt(),
    );

Map<String, dynamic> _$ProductOptionInputToJson(ProductOptionInput instance) =>
    <String, dynamic>{
      'active': ?instance.active,
      'default_quantity': ?instance.defaultQuantity,
      'option_product_id': instance.optionProductId,
      'required': ?instance.requiredValue,
      'sort_order': ?instance.sortOrder,
    };
