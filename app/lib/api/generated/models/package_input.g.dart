// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'package_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

PackageInput _$PackageInputFromJson(Map<String, dynamic> json) => PackageInput(
  carrier: json['carrier'] as String?,
  clientId: json['client_id'] as String?,
  condition: json['condition'] as String?,
  conditionNote: json['condition_note'] as String?,
  customerId: json['customer_id'] as String?,
  customerReference: json['customer_reference'] as String?,
  notes: json['notes'] as String?,
  parcelCount: (json['parcel_count'] as num?)?.toInt(),
  senderName: json['sender_name'] as String?,
  trackingNumber: json['tracking_number'] as String?,
);

Map<String, dynamic> _$PackageInputToJson(PackageInput instance) =>
    <String, dynamic>{
      'carrier': ?instance.carrier,
      'client_id': ?instance.clientId,
      'condition': ?instance.condition,
      'condition_note': ?instance.conditionNote,
      'customer_id': ?instance.customerId,
      'customer_reference': ?instance.customerReference,
      'notes': ?instance.notes,
      'parcel_count': ?instance.parcelCount,
      'sender_name': ?instance.senderName,
      'tracking_number': ?instance.trackingNumber,
    };
