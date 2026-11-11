// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'convert_ticket_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ConvertTicketInput _$ConvertTicketInputFromJson(Map<String, dynamic> json) =>
    ConvertTicketInput(
      customerReference: json['customer_reference'] as String?,
      notes: json['notes'] as String?,
    );

Map<String, dynamic> _$ConvertTicketInputToJson(ConvertTicketInput instance) =>
    <String, dynamic>{
      'customer_reference': ?instance.customerReference,
      'notes': ?instance.notes,
    };
