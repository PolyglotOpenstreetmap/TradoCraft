// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'product_option.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ProductOption _$ProductOptionFromJson(Map<String, dynamic> json) =>
    ProductOption(
      active: json['active'] as bool,
      defaultQuantity: json['default_quantity'] as String,
      optionActive: json['option_active'] as bool,
      optionCode: json['option_code'] as String,
      optionName: json['option_name'] as String,
      optionProductId: json['option_product_id'] as String,
      requiredValue: json['required'] as bool,
      sortOrder: (json['sort_order'] as num).toInt(),
    );

Map<String, dynamic> _$ProductOptionToJson(ProductOption instance) =>
    <String, dynamic>{
      'active': instance.active,
      'default_quantity': instance.defaultQuantity,
      'option_active': instance.optionActive,
      'option_code': instance.optionCode,
      'option_name': instance.optionName,
      'option_product_id': instance.optionProductId,
      'required': instance.requiredValue,
      'sort_order': instance.sortOrder,
    };
