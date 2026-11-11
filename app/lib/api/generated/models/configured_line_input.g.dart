// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'configured_line_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ConfiguredLineInput _$ConfiguredLineInputFromJson(Map<String, dynamic> json) =>
    ConfiguredLineInput(
      code: json['code'] as String,
      quantity: json['quantity'] as String,
      children: (json['children'] as List<dynamic>?)
          ?.map((e) => ConfiguredLineInput.fromJson(e as Map<String, dynamic>))
          .toList(),
      comment: json['comment'] as String?,
      description: json['description'] as String?,
      percentBase: json['percent_base'] == null
          ? null
          : PercentBase.fromJson(json['percent_base'] as String),
      percentOfParent: json['percent_of_parent'] as String?,
      section: json['section'] as String?,
      unitPrice: json['unit_price'] as String?,
    );

Map<String, dynamic> _$ConfiguredLineInputToJson(
  ConfiguredLineInput instance,
) => <String, dynamic>{
  'children': ?instance.children,
  'code': instance.code,
  'comment': ?instance.comment,
  'description': ?instance.description,
  'percent_base': ?instance.percentBase,
  'percent_of_parent': ?instance.percentOfParent,
  'quantity': instance.quantity,
  'section': ?instance.section,
  'unit_price': ?instance.unitPrice,
};
