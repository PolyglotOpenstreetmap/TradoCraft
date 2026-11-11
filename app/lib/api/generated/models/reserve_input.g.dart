// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'reserve_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ReserveInput _$ReserveInputFromJson(Map<String, dynamic> json) => ReserveInput(
  quantity: json['quantity'] as String,
  locationId: json['location_id'] as String?,
  lotId: json['lot_id'] as String?,
  warehouseId: json['warehouse_id'] as String?,
);

Map<String, dynamic> _$ReserveInputToJson(ReserveInput instance) =>
    <String, dynamic>{
      'location_id': ?instance.locationId,
      'lot_id': ?instance.lotId,
      'quantity': instance.quantity,
      'warehouse_id': ?instance.warehouseId,
    };
