// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'part_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

PartInput _$PartInputFromJson(Map<String, dynamic> json) => PartInput(
  quantity: json['quantity'] as String,
  billable: json['billable'] as bool?,
  description: json['description'] as String?,
  id: json['id'] as String?,
  productId: json['product_id'] as String?,
  serialUnitId: json['serial_unit_id'] as String?,
  unit: json['unit'] as String?,
);

Map<String, dynamic> _$PartInputToJson(PartInput instance) => <String, dynamic>{
  'billable': ?instance.billable,
  'description': ?instance.description,
  'id': ?instance.id,
  'product_id': ?instance.productId,
  'quantity': instance.quantity,
  'serial_unit_id': ?instance.serialUnitId,
  'unit': ?instance.unit,
};
