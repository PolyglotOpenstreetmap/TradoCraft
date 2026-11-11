// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'rebate_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

RebateInput _$RebateInputFromJson(Map<String, dynamic> json) => RebateInput(
  percent: json['percent'] as String,
  scope: json['scope'] as String,
  familyId: json['family_id'] as String?,
  notes: json['notes'] as String?,
  productId: json['product_id'] as String?,
  validFrom: json['valid_from'] == null
      ? null
      : DateTime.parse(json['valid_from'] as String),
  validTo: json['valid_to'] == null
      ? null
      : DateTime.parse(json['valid_to'] as String),
);

Map<String, dynamic> _$RebateInputToJson(RebateInput instance) =>
    <String, dynamic>{
      'family_id': ?instance.familyId,
      'notes': ?instance.notes,
      'percent': instance.percent,
      'product_id': ?instance.productId,
      'scope': instance.scope,
      'valid_from': ?instance.validFrom?.toIso8601String(),
      'valid_to': ?instance.validTo?.toIso8601String(),
    };
