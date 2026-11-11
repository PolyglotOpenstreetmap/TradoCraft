// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'repair.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Repair _$RepairFromJson(Map<String, dynamic> json) => Repair(
  createdAt: DateTime.parse(json['created_at'] as String),
  customerId: json['customer_id'] as String,
  customerName: json['customer_name'] as String,
  customerReference: json['customer_reference'] as String,
  diagnosis: json['diagnosis'] as String,
  faultDescription: json['fault_description'] as String,
  findings: json['findings'] as String,
  id: json['id'] as String,
  number: json['number'] as String,
  photoCount: (json['photo_count'] as num).toInt(),
  priority: json['priority'] as String,
  productDescription: json['product_description'] as String,
  serialNumber: json['serial_number'] as String,
  source: json['source'] as String,
  status: json['status'] as String,
  updatedAt: DateTime.parse(json['updated_at'] as String),
  warranty: json['warranty'] as String,
  warrantyAuto: json['warranty_auto'] as String,
  warrantyOverridden: json['warranty_overridden'] as bool,
  assignedName: json['assigned_name'] as String?,
  assignedUserId: json['assigned_user_id'] as String?,
  completedAt: json['completed_at'] == null
      ? null
      : DateTime.parse(json['completed_at'] as String),
  contactId: json['contact_id'] as String?,
  contactName: json['contact_name'] as String?,
  departmentId: json['department_id'] as String?,
  departmentName: json['department_name'] as String?,
  productCode: json['product_code'] as String?,
  productId: json['product_id'] as String?,
  receivedAt: json['received_at'] == null
      ? null
      : DateTime.parse(json['received_at'] as String),
  serialUnitId: json['serial_unit_id'] as String?,
  ticketId: json['ticket_id'] as String?,
  ticketNumber: json['ticket_number'] as String?,
);

Map<String, dynamic> _$RepairToJson(Repair instance) => <String, dynamic>{
  'assigned_name': ?instance.assignedName,
  'assigned_user_id': ?instance.assignedUserId,
  'completed_at': ?instance.completedAt?.toIso8601String(),
  'contact_id': ?instance.contactId,
  'contact_name': ?instance.contactName,
  'created_at': instance.createdAt.toIso8601String(),
  'customer_id': instance.customerId,
  'customer_name': instance.customerName,
  'customer_reference': instance.customerReference,
  'department_id': ?instance.departmentId,
  'department_name': ?instance.departmentName,
  'diagnosis': instance.diagnosis,
  'fault_description': instance.faultDescription,
  'findings': instance.findings,
  'id': instance.id,
  'number': instance.number,
  'photo_count': instance.photoCount,
  'priority': instance.priority,
  'product_code': ?instance.productCode,
  'product_description': instance.productDescription,
  'product_id': ?instance.productId,
  'received_at': ?instance.receivedAt?.toIso8601String(),
  'serial_number': instance.serialNumber,
  'serial_unit_id': ?instance.serialUnitId,
  'source': instance.source,
  'status': instance.status,
  'ticket_id': ?instance.ticketId,
  'ticket_number': ?instance.ticketNumber,
  'updated_at': instance.updatedAt.toIso8601String(),
  'warranty': instance.warranty,
  'warranty_auto': instance.warrantyAuto,
  'warranty_overridden': instance.warrantyOverridden,
};
