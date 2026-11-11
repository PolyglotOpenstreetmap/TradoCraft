// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'line.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Line _$LineFromJson(Map<String, dynamic> json) => Line(
  children: (json['children'] as List<dynamic>)
      .map((e) => Line.fromJson(e as Map<String, dynamic>))
      .toList(),
  code: json['code'] as String,
  quantity: json['quantity'] as String,
  rule: json['rule'] as String,
  trace: Trace.fromJson(json['trace'] as Map<String, dynamic>),
  comment: json['comment'] as String?,
  description: json['description'] as String?,
  percentBase: json['percent_base'] as String?,
  percentOfParent: json['percent_of_parent'] as String?,
  productId: json['product_id'] as String?,
  productName: json['product_name'] as String?,
  section: json['section'] as String?,
);

Map<String, dynamic> _$LineToJson(Line instance) => <String, dynamic>{
  'children': instance.children,
  'code': instance.code,
  'comment': ?instance.comment,
  'description': ?instance.description,
  'percent_base': ?instance.percentBase,
  'percent_of_parent': ?instance.percentOfParent,
  'product_id': ?instance.productId,
  'product_name': ?instance.productName,
  'quantity': instance.quantity,
  'rule': instance.rule,
  'section': ?instance.section,
  'trace': instance.trace,
};
