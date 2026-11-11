// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'repair_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

RepairInput _$RepairInputFromJson(Map<String, dynamic> json) => RepairInput(
  faultDescription: json['fault_description'] as String,
  priority: json['priority'] as String,
  warranty: json['warranty'] as String,
  contactId: json['contact_id'] as String?,
  customerReference: json['customer_reference'] as String?,
  productDescription: json['product_description'] as String?,
  productId: json['product_id'] as String?,
  serialNumber: json['serial_number'] as String?,
);

Map<String, dynamic> _$RepairInputToJson(RepairInput instance) =>
    <String, dynamic>{
      'contact_id': ?instance.contactId,
      'customer_reference': ?instance.customerReference,
      'fault_description': instance.faultDescription,
      'priority': instance.priority,
      'product_description': ?instance.productDescription,
      'product_id': ?instance.productId,
      'serial_number': ?instance.serialNumber,
      'warranty': instance.warranty,
    };
