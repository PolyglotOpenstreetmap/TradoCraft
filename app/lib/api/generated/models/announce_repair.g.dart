// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'announce_repair.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

AnnounceRepair _$AnnounceRepairFromJson(Map<String, dynamic> json) =>
    AnnounceRepair(
      faultDescription: json['fault_description'] as String,
      customerReference: json['customer_reference'] as String?,
      productDescription: json['product_description'] as String?,
      serialNumber: json['serial_number'] as String?,
    );

Map<String, dynamic> _$AnnounceRepairToJson(AnnounceRepair instance) =>
    <String, dynamic>{
      'customer_reference': ?instance.customerReference,
      'fault_description': instance.faultDescription,
      'product_description': ?instance.productDescription,
      'serial_number': ?instance.serialNumber,
    };
