// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'adjustment_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

AdjustmentInput _$AdjustmentInputFromJson(Map<String, dynamic> json) =>
    AdjustmentInput(
      id: json['id'] as String,
      locationId: json['location_id'] as String,
      note: json['note'] as String,
      productId: json['product_id'] as String,
      delta: json['delta'] as String?,
      expiryDate: json['expiry_date'] == null
          ? null
          : DateTime.parse(json['expiry_date'] as String),
      lotId: json['lot_id'] as String?,
      lotNumber: json['lot_number'] as String?,
      newQuantity: json['new_quantity'] as String?,
    );

Map<String, dynamic> _$AdjustmentInputToJson(AdjustmentInput instance) =>
    <String, dynamic>{
      'delta': ?instance.delta,
      'expiry_date': ?instance.expiryDate?.toIso8601String(),
      'id': instance.id,
      'location_id': instance.locationId,
      'lot_id': ?instance.lotId,
      'lot_number': ?instance.lotNumber,
      'new_quantity': ?instance.newQuantity,
      'note': instance.note,
      'product_id': instance.productId,
    };
