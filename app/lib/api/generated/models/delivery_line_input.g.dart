// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'delivery_line_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

DeliveryLineInput _$DeliveryLineInputFromJson(Map<String, dynamic> json) =>
    DeliveryLineInput(
      locationId: json['location_id'] as String,
      orderLineId: json['order_line_id'] as String,
      quantity: json['quantity'] as String,
      lotId: json['lot_id'] as String?,
      serialUnitId: json['serial_unit_id'] as String?,
    );

Map<String, dynamic> _$DeliveryLineInputToJson(DeliveryLineInput instance) =>
    <String, dynamic>{
      'location_id': instance.locationId,
      'lot_id': ?instance.lotId,
      'order_line_id': instance.orderLineId,
      'quantity': instance.quantity,
      'serial_unit_id': ?instance.serialUnitId,
    };
