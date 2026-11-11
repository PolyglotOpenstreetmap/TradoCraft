// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'new_repair_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

NewRepairInput _$NewRepairInputFromJson(Map<String, dynamic> json) =>
    NewRepairInput(
      announced: json['announced'] as bool?,
      contactId: json['contact_id'] as String?,
      customerId: json['customer_id'] as String?,
      customerReference: json['customer_reference'] as String?,
      faultDescription: json['fault_description'] as String?,
      priority: json['priority'] as String?,
      productDescription: json['product_description'] as String?,
      productId: json['product_id'] as String?,
      serialNumber: json['serial_number'] as String?,
      ticketId: json['ticket_id'] as String?,
      warranty: json['warranty'] as String?,
    );

Map<String, dynamic> _$NewRepairInputToJson(NewRepairInput instance) =>
    <String, dynamic>{
      'announced': ?instance.announced,
      'contact_id': ?instance.contactId,
      'customer_id': ?instance.customerId,
      'customer_reference': ?instance.customerReference,
      'fault_description': ?instance.faultDescription,
      'priority': ?instance.priority,
      'product_description': ?instance.productDescription,
      'product_id': ?instance.productId,
      'serial_number': ?instance.serialNumber,
      'ticket_id': ?instance.ticketId,
      'warranty': ?instance.warranty,
    };
