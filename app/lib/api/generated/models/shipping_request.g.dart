// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'shipping_request.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ShippingRequest _$ShippingRequestFromJson(Map<String, dynamic> json) =>
    ShippingRequest(
      createdAt: DateTime.parse(json['created_at'] as String),
      customerName: json['customer_name'] as String,
      id: json['id'] as String,
      notes: json['notes'] as String,
      quotationId: json['quotation_id'] as String,
      quotationNumber: json['quotation_number'] as String,
      quotationRevision: (json['quotation_revision'] as num).toInt(),
      status: json['status'] as String,
      carrier: json['carrier'] as String?,
      currency: json['currency'] as String?,
      deliveryCountryCode: json['delivery_country_code'] as String?,
      quotedAmount: json['quoted_amount'] as String?,
      quotedAt: json['quoted_at'] == null
          ? null
          : DateTime.parse(json['quoted_at'] as String),
      quotedBy: json['quoted_by'] as String?,
      requestedAt: json['requested_at'] == null
          ? null
          : DateTime.parse(json['requested_at'] as String),
      requestedBy: json['requested_by'] as String?,
    );

Map<String, dynamic> _$ShippingRequestToJson(ShippingRequest instance) =>
    <String, dynamic>{
      'carrier': ?instance.carrier,
      'created_at': instance.createdAt.toIso8601String(),
      'currency': ?instance.currency,
      'customer_name': instance.customerName,
      'delivery_country_code': ?instance.deliveryCountryCode,
      'id': instance.id,
      'notes': instance.notes,
      'quotation_id': instance.quotationId,
      'quotation_number': instance.quotationNumber,
      'quotation_revision': instance.quotationRevision,
      'quoted_amount': ?instance.quotedAmount,
      'quoted_at': ?instance.quotedAt?.toIso8601String(),
      'quoted_by': ?instance.quotedBy,
      'requested_at': ?instance.requestedAt?.toIso8601String(),
      'requested_by': ?instance.requestedBy,
      'status': instance.status,
    };
