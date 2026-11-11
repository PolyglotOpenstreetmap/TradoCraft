// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'quotation_summary.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

QuotationSummary _$QuotationSummaryFromJson(Map<String, dynamic> json) =>
    QuotationSummary(
      currency: json['currency'] as String,
      customerId: json['customer_id'] as String,
      customerName: json['customer_name'] as String,
      id: json['id'] as String,
      number: json['number'] as String,
      purpose: json['purpose'] as String,
      quoteDate: DateTime.parse(json['quote_date'] as String),
      revision: (json['revision'] as num).toInt(),
      shippingStatus: json['shipping_status'] as String,
      status: QuotationStatus.fromJson(json['status'] as String),
      subtotal: json['subtotal'] as String,
      total: json['total'] as String,
      validUntil: DateTime.parse(json['valid_until'] as String),
      opportunityId: json['opportunity_id'] as String?,
    );

Map<String, dynamic> _$QuotationSummaryToJson(QuotationSummary instance) =>
    <String, dynamic>{
      'currency': instance.currency,
      'customer_id': instance.customerId,
      'customer_name': instance.customerName,
      'id': instance.id,
      'number': instance.number,
      'opportunity_id': ?instance.opportunityId,
      'purpose': instance.purpose,
      'quote_date': instance.quoteDate.toIso8601String(),
      'revision': instance.revision,
      'shipping_status': instance.shippingStatus,
      'status': instance.status,
      'subtotal': instance.subtotal,
      'total': instance.total,
      'valid_until': instance.validUntil.toIso8601String(),
    };
