// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'new_purchase_order_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

NewPurchaseOrderInput _$NewPurchaseOrderInputFromJson(
  Map<String, dynamic> json,
) => NewPurchaseOrderInput(
  lines: (json['lines'] as List<dynamic>)
      .map((e) => NewPoLineInput.fromJson(e as Map<String, dynamic>))
      .toList(),
  supplierId: json['supplier_id'] as String,
  currency: json['currency'] as String?,
  expectedDeliveryDate: json['expected_delivery_date'] == null
      ? null
      : DateTime.parse(json['expected_delivery_date'] as String),
  notes: json['notes'] as String?,
  orderDate: json['order_date'] == null
      ? null
      : DateTime.parse(json['order_date'] as String),
);

Map<String, dynamic> _$NewPurchaseOrderInputToJson(
  NewPurchaseOrderInput instance,
) => <String, dynamic>{
  'currency': ?instance.currency,
  'expected_delivery_date': ?instance.expectedDeliveryDate?.toIso8601String(),
  'lines': instance.lines,
  'notes': ?instance.notes,
  'order_date': ?instance.orderDate?.toIso8601String(),
  'supplier_id': instance.supplierId,
};
