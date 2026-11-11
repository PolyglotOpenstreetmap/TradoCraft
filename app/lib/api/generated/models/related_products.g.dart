// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'related_products.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

RelatedProducts _$RelatedProductsFromJson(Map<String, dynamic> json) =>
    RelatedProducts(
      rules: (json['rules'] as List<dynamic>)
          .map((e) => ProductRelation.fromJson(e as Map<String, dynamic>))
          .toList(),
      suggestions: (json['suggestions'] as List<dynamic>)
          .map((e) => Suggestion.fromJson(e as Map<String, dynamic>))
          .toList(),
    );

Map<String, dynamic> _$RelatedProductsToJson(RelatedProducts instance) =>
    <String, dynamic>{
      'rules': instance.rules,
      'suggestions': instance.suggestions,
    };
