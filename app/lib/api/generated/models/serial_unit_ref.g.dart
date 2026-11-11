// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'serial_unit_ref.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

SerialUnitRef _$SerialUnitRefFromJson(Map<String, dynamic> json) =>
    SerialUnitRef(
      id: json['id'] as String,
      productCode: json['product_code'] as String,
      productId: json['product_id'] as String,
      productName: json['product_name'] as String,
      serialNumber: json['serial_number'] as String,
      customerId: json['customer_id'] as String?,
      warrantyUntil: json['warranty_until'] == null
          ? null
          : DateTime.parse(json['warranty_until'] as String),
    );

Map<String, dynamic> _$SerialUnitRefToJson(SerialUnitRef instance) =>
    <String, dynamic>{
      'customer_id': ?instance.customerId,
      'id': instance.id,
      'product_code': instance.productCode,
      'product_id': instance.productId,
      'product_name': instance.productName,
      'serial_number': instance.serialNumber,
      'warranty_until': ?instance.warrantyUntil?.toIso8601String(),
    };
