// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'serial_unit.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

SerialUnit _$SerialUnitFromJson(Map<String, dynamic> json) => SerialUnit(
  createdAt: DateTime.parse(json['created_at'] as String),
  id: json['id'] as String,
  notes: json['notes'] as String,
  productCode: json['product_code'] as String,
  productId: json['product_id'] as String,
  productName: json['product_name'] as String,
  serialNumber: json['serial_number'] as String,
  status: json['status'] as String,
  updatedAt: DateTime.parse(json['updated_at'] as String),
  warrantyNote: json['warranty_note'] as String,
  warrantyStatus: json['warranty_status'] as String,
  bomId: json['bom_id'] as String?,
  bomRevision: (json['bom_revision'] as num?)?.toInt(),
  customerId: json['customer_id'] as String?,
  customerName: json['customer_name'] as String?,
  parentId: json['parent_id'] as String?,
  parentSerialNumber: json['parent_serial_number'] as String?,
  warrantyUntil: json['warranty_until'] == null
      ? null
      : DateTime.parse(json['warranty_until'] as String),
);

Map<String, dynamic> _$SerialUnitToJson(SerialUnit instance) =>
    <String, dynamic>{
      'bom_id': ?instance.bomId,
      'bom_revision': ?instance.bomRevision,
      'created_at': instance.createdAt.toIso8601String(),
      'customer_id': ?instance.customerId,
      'customer_name': ?instance.customerName,
      'id': instance.id,
      'notes': instance.notes,
      'parent_id': ?instance.parentId,
      'parent_serial_number': ?instance.parentSerialNumber,
      'product_code': instance.productCode,
      'product_id': instance.productId,
      'product_name': instance.productName,
      'serial_number': instance.serialNumber,
      'status': instance.status,
      'updated_at': instance.updatedAt.toIso8601String(),
      'warranty_note': instance.warrantyNote,
      'warranty_status': instance.warrantyStatus,
      'warranty_until': ?instance.warrantyUntil?.toIso8601String(),
    };
