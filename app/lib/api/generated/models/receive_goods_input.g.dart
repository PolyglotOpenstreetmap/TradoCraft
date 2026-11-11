// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'receive_goods_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ReceiveGoodsInput _$ReceiveGoodsInputFromJson(Map<String, dynamic> json) =>
    ReceiveGoodsInput(
      lines: (json['lines'] as List<dynamic>)
          .map((e) => ReceiveGoodsLineInput.fromJson(e as Map<String, dynamic>))
          .toList(),
      locationId: json['location_id'] as String,
      warehouseId: json['warehouse_id'] as String,
      carrier: json['carrier'] as String?,
      notes: json['notes'] as String?,
      supplierDeliveryNote: json['supplier_delivery_note'] as String?,
      trackingNumber: json['tracking_number'] as String?,
    );

Map<String, dynamic> _$ReceiveGoodsInputToJson(ReceiveGoodsInput instance) =>
    <String, dynamic>{
      'carrier': ?instance.carrier,
      'lines': instance.lines,
      'location_id': instance.locationId,
      'notes': ?instance.notes,
      'supplier_delivery_note': ?instance.supplierDeliveryNote,
      'tracking_number': ?instance.trackingNumber,
      'warehouse_id': instance.warehouseId,
    };
