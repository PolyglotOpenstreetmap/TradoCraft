// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'serial_status_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

SerialStatusInput _$SerialStatusInputFromJson(Map<String, dynamic> json) =>
    SerialStatusInput(
      status: json['status'] as String,
      customerId: json['customer_id'] as String?,
    );

Map<String, dynamic> _$SerialStatusInputToJson(SerialStatusInput instance) =>
    <String, dynamic>{
      'customer_id': ?instance.customerId,
      'status': instance.status,
    };
