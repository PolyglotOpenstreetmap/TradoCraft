// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'job_part.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

JobPart _$JobPartFromJson(Map<String, dynamic> json) => JobPart(
  billable: json['billable'] as bool,
  createdAt: DateTime.parse(json['created_at'] as String),
  description: json['description'] as String,
  id: json['id'] as String,
  quantity: json['quantity'] as String,
  unit: json['unit'] as String,
  addedByName: json['added_by_name'] as String?,
  productCode: json['product_code'] as String?,
  productId: json['product_id'] as String?,
  serialNumber: json['serial_number'] as String?,
  serialUnitId: json['serial_unit_id'] as String?,
);

Map<String, dynamic> _$JobPartToJson(JobPart instance) => <String, dynamic>{
  'added_by_name': ?instance.addedByName,
  'billable': instance.billable,
  'created_at': instance.createdAt.toIso8601String(),
  'description': instance.description,
  'id': instance.id,
  'product_code': ?instance.productCode,
  'product_id': ?instance.productId,
  'quantity': instance.quantity,
  'serial_number': ?instance.serialNumber,
  'serial_unit_id': ?instance.serialUnitId,
  'unit': instance.unit,
};
