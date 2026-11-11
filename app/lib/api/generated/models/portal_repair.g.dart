// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'portal_repair.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

PortalRepair _$PortalRepairFromJson(Map<String, dynamic> json) => PortalRepair(
  contactName: json['contact_name'] as String,
  createdAt: DateTime.parse(json['created_at'] as String),
  customerReference: json['customer_reference'] as String,
  faultDescription: json['fault_description'] as String,
  id: json['id'] as String,
  number: json['number'] as String,
  photoCount: (json['photo_count'] as num).toInt(),
  productDescription: json['product_description'] as String,
  serialNumber: json['serial_number'] as String,
  status: json['status'] as String,
  updatedAt: DateTime.parse(json['updated_at'] as String),
);

Map<String, dynamic> _$PortalRepairToJson(PortalRepair instance) =>
    <String, dynamic>{
      'contact_name': instance.contactName,
      'created_at': instance.createdAt.toIso8601String(),
      'customer_reference': instance.customerReference,
      'fault_description': instance.faultDescription,
      'id': instance.id,
      'number': instance.number,
      'photo_count': instance.photoCount,
      'product_description': instance.productDescription,
      'serial_number': instance.serialNumber,
      'status': instance.status,
      'updated_at': instance.updatedAt.toIso8601String(),
    };
