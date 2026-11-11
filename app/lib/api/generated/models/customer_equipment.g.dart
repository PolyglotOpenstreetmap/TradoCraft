// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'customer_equipment.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

CustomerEquipment _$CustomerEquipmentFromJson(Map<String, dynamic> json) =>
    CustomerEquipment(
      productCode: json['product_code'] as String,
      productName: json['product_name'] as String,
      serialNumber: json['serial_number'] as String,
      serialUnitId: json['serial_unit_id'] as String,
      status: json['status'] as String,
      warrantyNote: json['warranty_note'] as String,
      warrantyStatus: json['warranty_status'] as String,
      jobs: (json['jobs'] as List<dynamic>)
          .map((e) => EquipmentJob.fromJson(e as Map<String, dynamic>))
          .toList(),
      warrantyUntil: json['warranty_until'] == null
          ? null
          : DateTime.parse(json['warranty_until'] as String),
    );

Map<String, dynamic> _$CustomerEquipmentToJson(CustomerEquipment instance) =>
    <String, dynamic>{
      'product_code': instance.productCode,
      'product_name': instance.productName,
      'serial_number': instance.serialNumber,
      'serial_unit_id': instance.serialUnitId,
      'status': instance.status,
      'warranty_note': instance.warrantyNote,
      'warranty_status': instance.warrantyStatus,
      'warranty_until': ?instance.warrantyUntil?.toIso8601String(),
      'jobs': instance.jobs,
    };
