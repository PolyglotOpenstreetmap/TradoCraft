// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'lot.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Lot _$LotFromJson(Map<String, dynamic> json) => Lot(
  id: json['id'] as String,
  lotNumber: json['lot_number'] as String,
  notes: json['notes'] as String,
  onHand: json['on_hand'] as String,
  productCode: json['product_code'] as String,
  productId: json['product_id'] as String,
  expiryDate: json['expiry_date'] == null
      ? null
      : DateTime.parse(json['expiry_date'] as String),
);

Map<String, dynamic> _$LotToJson(Lot instance) => <String, dynamic>{
  'expiry_date': ?instance.expiryDate?.toIso8601String(),
  'id': instance.id,
  'lot_number': instance.lotNumber,
  'notes': instance.notes,
  'on_hand': instance.onHand,
  'product_code': instance.productCode,
  'product_id': instance.productId,
};
