// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'delivery_note_line.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

DeliveryNoteLine _$DeliveryNoteLineFromJson(Map<String, dynamic> json) =>
    DeliveryNoteLine(
      id: json['id'] as String,
      location: json['location'] as String,
      locationId: json['location_id'] as String,
      orderLineId: json['order_line_id'] as String,
      orderPosition: (json['order_position'] as num).toInt(),
      position: (json['position'] as num).toInt(),
      productCode: json['product_code'] as String,
      productId: json['product_id'] as String,
      productName: json['product_name'] as String,
      quantity: json['quantity'] as String,
      tracking: json['tracking'] as String,
      unit: json['unit'] as String,
      lotId: json['lot_id'] as String?,
      lotNumber: json['lot_number'] as String?,
      serialNumber: json['serial_number'] as String?,
      serialUnitId: json['serial_unit_id'] as String?,
      stockMoveId: json['stock_move_id'] as String?,
    );

Map<String, dynamic> _$DeliveryNoteLineToJson(DeliveryNoteLine instance) =>
    <String, dynamic>{
      'id': instance.id,
      'location': instance.location,
      'location_id': instance.locationId,
      'lot_id': ?instance.lotId,
      'lot_number': ?instance.lotNumber,
      'order_line_id': instance.orderLineId,
      'order_position': instance.orderPosition,
      'position': instance.position,
      'product_code': instance.productCode,
      'product_id': instance.productId,
      'product_name': instance.productName,
      'quantity': instance.quantity,
      'serial_number': ?instance.serialNumber,
      'serial_unit_id': ?instance.serialUnitId,
      'stock_move_id': ?instance.stockMoveId,
      'tracking': instance.tracking,
      'unit': instance.unit,
    };
