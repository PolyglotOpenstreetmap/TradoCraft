// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'product_family_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ProductFamilyInput _$ProductFamilyInputFromJson(Map<String, dynamic> json) =>
    ProductFamilyInput(
      code: json['code'] as String,
      name: json['name'] as String,
      active: json['active'] as bool?,
    );

Map<String, dynamic> _$ProductFamilyInputToJson(ProductFamilyInput instance) =>
    <String, dynamic>{
      'active': ?instance.active,
      'code': instance.code,
      'name': instance.name,
    };
