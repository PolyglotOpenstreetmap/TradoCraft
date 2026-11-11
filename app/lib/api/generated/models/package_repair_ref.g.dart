// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'package_repair_ref.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

PackageRepairRef _$PackageRepairRefFromJson(Map<String, dynamic> json) =>
    PackageRepairRef(
      customerName: json['customer_name'] as String,
      id: json['id'] as String,
      linkedAt: DateTime.parse(json['linked_at'] as String),
      number: json['number'] as String,
      productDescription: json['product_description'] as String,
      serialNumber: json['serial_number'] as String,
      status: json['status'] as String,
      linkedByName: json['linked_by_name'] as String?,
    );

Map<String, dynamic> _$PackageRepairRefToJson(PackageRepairRef instance) =>
    <String, dynamic>{
      'customer_name': instance.customerName,
      'id': instance.id,
      'linked_at': instance.linkedAt.toIso8601String(),
      'linked_by_name': ?instance.linkedByName,
      'number': instance.number,
      'product_description': instance.productDescription,
      'serial_number': instance.serialNumber,
      'status': instance.status,
    };
