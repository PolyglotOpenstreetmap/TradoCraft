// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'shipping_quote_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ShippingQuoteInput _$ShippingQuoteInputFromJson(Map<String, dynamic> json) =>
    ShippingQuoteInput(
      amount: json['amount'] as String,
      carrier: json['carrier'] as String?,
      currency: json['currency'] as String?,
      notes: json['notes'] as String?,
    );

Map<String, dynamic> _$ShippingQuoteInputToJson(ShippingQuoteInput instance) =>
    <String, dynamic>{
      'amount': instance.amount,
      'carrier': ?instance.carrier,
      'currency': ?instance.currency,
      'notes': ?instance.notes,
    };
