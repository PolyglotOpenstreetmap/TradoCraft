// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'opportunity_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

OpportunityInput _$OpportunityInputFromJson(Map<String, dynamic> json) =>
    OpportunityInput(
      customerId: json['customer_id'] as String,
      title: json['title'] as String,
      expectedClose: json['expected_close'] == null
          ? null
          : DateTime.parse(json['expected_close'] as String),
      expectedValue: json['expected_value'] as String?,
      notes: json['notes'] as String?,
      ownerUserId: json['owner_user_id'] as String?,
      probability: (json['probability'] as num?)?.toInt(),
    );

Map<String, dynamic> _$OpportunityInputToJson(OpportunityInput instance) =>
    <String, dynamic>{
      'customer_id': instance.customerId,
      'expected_close': ?instance.expectedClose?.toIso8601String(),
      'expected_value': ?instance.expectedValue,
      'notes': ?instance.notes,
      'owner_user_id': ?instance.ownerUserId,
      'probability': ?instance.probability,
      'title': instance.title,
    };
