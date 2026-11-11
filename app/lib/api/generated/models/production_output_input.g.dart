// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'production_output_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ProductionOutputInput _$ProductionOutputInputFromJson(
  Map<String, dynamic> json,
) => ProductionOutputInput(
  locationId: json['location_id'] as String,
  quantity: json['quantity'] as String,
  expiryDate: json['expiry_date'] == null
      ? null
      : DateTime.parse(json['expiry_date'] as String),
  lotNumber: json['lot_number'] as String?,
  serialNumbers: (json['serial_numbers'] as List<dynamic>?)
      ?.map((e) => e as String)
      .toList(),
);

Map<String, dynamic> _$ProductionOutputInputToJson(
  ProductionOutputInput instance,
) => <String, dynamic>{
  'expiry_date': ?instance.expiryDate?.toIso8601String(),
  'location_id': instance.locationId,
  'lot_number': ?instance.lotNumber,
  'quantity': instance.quantity,
  'serial_numbers': ?instance.serialNumbers,
};
