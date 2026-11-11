// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'order_line_edit.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

OrderLineEdit _$OrderLineEditFromJson(Map<String, dynamic> json) =>
    OrderLineEdit(
      position: (json['position'] as num).toInt(),
      quantity: json['quantity'] as String,
      description: json['description'] as String?,
      lineDiscountPercent: json['line_discount_percent'] as String?,
    );

Map<String, dynamic> _$OrderLineEditToJson(OrderLineEdit instance) =>
    <String, dynamic>{
      'description': ?instance.description,
      'line_discount_percent': ?instance.lineDiscountPercent,
      'position': instance.position,
      'quantity': instance.quantity,
    };
