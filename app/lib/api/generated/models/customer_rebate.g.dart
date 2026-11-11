// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'customer_rebate.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

CustomerRebate _$CustomerRebateFromJson(Map<String, dynamic> json) =>
    CustomerRebate(
      createdAt: DateTime.parse(json['created_at'] as String),
      customerId: json['customer_id'] as String,
      id: json['id'] as String,
      notes: json['notes'] as String,
      percent: json['percent'] as String,
      scope: json['scope'] as String,
      familyCode: json['family_code'] as String?,
      familyId: json['family_id'] as String?,
      productCode: json['product_code'] as String?,
      productId: json['product_id'] as String?,
      validFrom: json['valid_from'] == null
          ? null
          : DateTime.parse(json['valid_from'] as String),
      validTo: json['valid_to'] == null
          ? null
          : DateTime.parse(json['valid_to'] as String),
    );

Map<String, dynamic> _$CustomerRebateToJson(CustomerRebate instance) =>
    <String, dynamic>{
      'created_at': instance.createdAt.toIso8601String(),
      'customer_id': instance.customerId,
      'family_code': ?instance.familyCode,
      'family_id': ?instance.familyId,
      'id': instance.id,
      'notes': instance.notes,
      'percent': instance.percent,
      'product_code': ?instance.productCode,
      'product_id': ?instance.productId,
      'scope': instance.scope,
      'valid_from': ?instance.validFrom?.toIso8601String(),
      'valid_to': ?instance.validTo?.toIso8601String(),
    };
