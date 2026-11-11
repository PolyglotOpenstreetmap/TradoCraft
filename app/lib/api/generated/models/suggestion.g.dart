// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'suggestion.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Suggestion _$SuggestionFromJson(Map<String, dynamic> json) => Suggestion(
  becauseOfCode: json['because_of_code'] as String,
  note: json['note'] as String,
  productCode: json['product_code'] as String,
  productId: json['product_id'] as String,
  productName: json['product_name'] as String,
  reason: SuggestionReason.fromJson(json['reason'] as String),
  confidence: (json['confidence'] as num?)?.toDouble(),
  strength: (json['strength'] as num?)?.toInt(),
);

Map<String, dynamic> _$SuggestionToJson(Suggestion instance) =>
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
