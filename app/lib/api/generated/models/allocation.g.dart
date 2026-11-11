// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'allocation.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Allocation _$AllocationFromJson(Map<String, dynamic> json) => Allocation(
  createdAt: DateTime.parse(json['created_at'] as String),
  id: json['id'] as String,
  location: json['location'] as String,
  locationId: json['location_id'] as String,
  quantity: json['quantity'] as String,
  lotId: json['lot_id'] as String?,
  lotNumber: json['lot_number'] as String?,
);

Map<String, dynamic> _$AllocationToJson(Allocation instance) =>
    <String, dynamic>{
      'created_at': instance.createdAt.toIso8601String(),
      'id': instance.id,
      'location': instance.location,
      'location_id': instance.locationId,
      'lot_id': ?instance.lotId,
      'lot_number': ?instance.lotNumber,
      'quantity': instance.quantity,
    };
