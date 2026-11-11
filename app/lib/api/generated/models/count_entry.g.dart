// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'count_entry.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

CountEntry _$CountEntryFromJson(Map<String, dynamic> json) => CountEntry(
  countedQuantity: json['counted_quantity'] as String?,
  expiryDate: json['expiry_date'] == null
      ? null
      : DateTime.parse(json['expiry_date'] as String),
  lineId: json['line_id'] as String?,
  locationId: json['location_id'] as String?,
  lotId: json['lot_id'] as String?,
  lotNumber: json['lot_number'] as String?,
  productId: json['product_id'] as String?,
);

Map<String, dynamic> _$CountEntryToJson(CountEntry instance) =>
    <String, dynamic>{
      'counted_quantity': ?instance.countedQuantity,
      'expiry_date': ?instance.expiryDate?.toIso8601String(),
      'line_id': ?instance.lineId,
      'location_id': ?instance.locationId,
      'lot_id': ?instance.lotId,
      'lot_number': ?instance.lotNumber,
      'product_id': ?instance.productId,
    };
