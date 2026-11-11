// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'package.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Package _$PackageFromJson(Map<String, dynamic> json) => Package(
  carrier: json['carrier'] as String,
  condition: json['condition'] as String,
  conditionNote: json['condition_note'] as String,
  createdAt: DateTime.parse(json['created_at'] as String),
  customerReference: json['customer_reference'] as String,
  id: json['id'] as String,
  notes: json['notes'] as String,
  number: json['number'] as String,
  parcelCount: (json['parcel_count'] as num).toInt(),
  photoCount: (json['photo_count'] as num).toInt(),
  receivedAt: DateTime.parse(json['received_at'] as String),
  receivedBy: json['received_by'] as String,
  receivedByName: json['received_by_name'] as String,
  repairCount: (json['repair_count'] as num).toInt(),
  senderName: json['sender_name'] as String,
  status: json['status'] as String,
  trackingNumber: json['tracking_number'] as String,
  updatedAt: DateTime.parse(json['updated_at'] as String),
  customerId: json['customer_id'] as String?,
  customerName: json['customer_name'] as String?,
  kind: json['kind'] as String?,
  openedAt: json['opened_at'] == null
      ? null
      : DateTime.parse(json['opened_at'] as String),
  openedBy: json['opened_by'] as String?,
  openedByName: json['opened_by_name'] as String?,
);

Map<String, dynamic> _$PackageToJson(Package instance) => <String, dynamic>{
  'carrier': instance.carrier,
  'condition': instance.condition,
  'condition_note': instance.conditionNote,
  'created_at': instance.createdAt.toIso8601String(),
  'customer_id': ?instance.customerId,
  'customer_name': ?instance.customerName,
  'customer_reference': instance.customerReference,
  'id': instance.id,
  'kind': ?instance.kind,
  'notes': instance.notes,
  'number': instance.number,
  'opened_at': ?instance.openedAt?.toIso8601String(),
  'opened_by': ?instance.openedBy,
  'opened_by_name': ?instance.openedByName,
  'parcel_count': instance.parcelCount,
  'photo_count': instance.photoCount,
  'received_at': instance.receivedAt.toIso8601String(),
  'received_by': instance.receivedBy,
  'received_by_name': instance.receivedByName,
  'repair_count': instance.repairCount,
  'sender_name': instance.senderName,
  'status': instance.status,
  'tracking_number': instance.trackingNumber,
  'updated_at': instance.updatedAt.toIso8601String(),
};
