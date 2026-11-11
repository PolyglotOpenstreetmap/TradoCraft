// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'package_new_repair_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

PackageNewRepairInput _$PackageNewRepairInputFromJson(
  Map<String, dynamic> json,
) => PackageNewRepairInput(
  contactId: json['contact_id'] as String?,
  customerId: json['customer_id'] as String?,
  customerReference: json['customer_reference'] as String?,
  faultDescription: json['fault_description'] as String?,
  productDescription: json['product_description'] as String?,
  productId: json['product_id'] as String?,
  serialNumber: json['serial_number'] as String?,
  ticketId: json['ticket_id'] as String?,
);

Map<String, dynamic> _$PackageNewRepairInputToJson(
  PackageNewRepairInput instance,
) => <String, dynamic>{
  'contact_id': ?instance.contactId,
  'customer_id': ?instance.customerId,
  'customer_reference': ?instance.customerReference,
  'fault_description': ?instance.faultDescription,
  'product_description': ?instance.productDescription,
  'product_id': ?instance.productId,
  'serial_number': ?instance.serialNumber,
  'ticket_id': ?instance.ticketId,
};
