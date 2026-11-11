// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'transfer_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

TransferInput _$TransferInputFromJson(Map<String, dynamic> json) =>
    TransferInput(
      fromLocationId: json['from_location_id'] as String,
      id: json['id'] as String,
      productId: json['product_id'] as String,
      quantity: json['quantity'] as String,
      toLocationId: json['to_location_id'] as String,
      lotId: json['lot_id'] as String?,
      note: json['note'] as String?,
      reason: json['reason'] as String?,
      serialUnitId: json['serial_unit_id'] as String?,
      uom: json['uom'] as String?,
    );

Map<String, dynamic> _$TransferInputToJson(TransferInput instance) =>
    <String, dynamic>{
      'from_location_id': instance.fromLocationId,
      'id': instance.id,
      'lot_id': ?instance.lotId,
      'note': ?instance.note,
      'product_id': instance.productId,
      'quantity': instance.quantity,
      'reason': ?instance.reason,
      'serial_unit_id': ?instance.serialUnitId,
      'to_location_id': instance.toLocationId,
      'uom': ?instance.uom,
    };
