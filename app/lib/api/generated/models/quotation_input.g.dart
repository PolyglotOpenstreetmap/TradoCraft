// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'quotation_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

QuotationInput _$QuotationInputFromJson(Map<String, dynamic> json) =>
    QuotationInput(
      customerId: json['customer_id'] as String,
      lines: (json['lines'] as List<dynamic>)
          .map((e) => QuotationLineInput.fromJson(e as Map<String, dynamic>))
          .toList(),
      clearPowerGrid: json['clear_power_grid'] as bool?,
      deliveryCountryCode: json['delivery_country_code'] as String?,
      notes: json['notes'] as String?,
      opportunityId: json['opportunity_id'] as String?,
      powerGridId: json['power_grid_id'] as String?,
      priceDate: json['price_date'] == null
          ? null
          : DateTime.parse(json['price_date'] as String),
      purpose: json['purpose'] as String?,
      quoteDate: json['quote_date'] == null
          ? null
          : DateTime.parse(json['quote_date'] as String),
      validDays: (json['valid_days'] as num?)?.toInt(),
    );

Map<String, dynamic> _$QuotationInputToJson(QuotationInput instance) =>
    <String, dynamic>{
      'clear_power_grid': ?instance.clearPowerGrid,
      'customer_id': instance.customerId,
      'delivery_country_code': ?instance.deliveryCountryCode,
      'lines': instance.lines,
      'notes': ?instance.notes,
      'opportunity_id': ?instance.opportunityId,
      'power_grid_id': ?instance.powerGridId,
      'price_date': ?instance.priceDate?.toIso8601String(),
      'purpose': ?instance.purpose,
      'quote_date': ?instance.quoteDate?.toIso8601String(),
      'valid_days': ?instance.validDays,
    };
