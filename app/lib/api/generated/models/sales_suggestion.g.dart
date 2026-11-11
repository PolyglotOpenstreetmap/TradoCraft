// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'sales_suggestion.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

SalesSuggestion _$SalesSuggestionFromJson(Map<String, dynamic> json) =>
    SalesSuggestion(
      becauseOfCode: json['because_of_code'] as String,
      note: json['note'] as String,
      productCode: json['product_code'] as String,
      productId: json['product_id'] as String,
      productName: json['product_name'] as String,
      reason: json['reason'] as String,
      confidence: (json['confidence'] as num?)?.toDouble(),
      strength: (json['strength'] as num?)?.toInt(),
    );

Map<String, dynamic> _$SalesSuggestionToJson(SalesSuggestion instance) =>
    <String, dynamic>{
      'because_of_code': instance.becauseOfCode,
      'confidence': ?instance.confidence,
      'note': instance.note,
      'product_code': instance.productCode,
      'product_id': instance.productId,
      'product_name': instance.productName,
      'reason': instance.reason,
      'strength': ?instance.strength,
    };
