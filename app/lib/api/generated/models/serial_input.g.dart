// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'serial_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

SerialInput _$SerialInputFromJson(Map<String, dynamic> json) => SerialInput(
  serialNumber: json['serial_number'] as String,
  bomId: json['bom_id'] as String?,
  customerId: json['customer_id'] as String?,
  notes: json['notes'] as String?,
);

Map<String, dynamic> _$SerialInputToJson(SerialInput instance) =>
    <String, dynamic>{
      'bom_id': ?instance.bomId,
      'customer_id': ?instance.customerId,
      'notes': ?instance.notes,
      'serial_number': instance.serialNumber,
    };
