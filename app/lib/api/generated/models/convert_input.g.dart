// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'convert_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ConvertInput _$ConvertInputFromJson(Map<String, dynamic> json) => ConvertInput(
  customerReference: json['customer_reference'] as String?,
  notes: json['notes'] as String?,
);

Map<String, dynamic> _$ConvertInputToJson(ConvertInput instance) =>
    <String, dynamic>{
      'customer_reference': ?instance.customerReference,
      'notes': ?instance.notes,
    };
