// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'repricing_comparison.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

RepricingComparison _$RepricingComparisonFromJson(Map<String, dynamic> json) =>
    RepricingComparison(
      difference: json['difference'] as String,
      lines: (json['lines'] as List<dynamic>)
          .map((e) => RepricingLine.fromJson(e as Map<String, dynamic>))
          .toList(),
      newPriceDate: DateTime.parse(json['new_price_date'] as String),
      newSubtotal: json['new_subtotal'] as String,
      oldPriceDate: DateTime.parse(json['old_price_date'] as String),
      oldSubtotal: json['old_subtotal'] as String,
      oldShippingTotal: json['old_shipping_total'] as String?,
      shippingRequoteNeeded: json['shipping_requote_needed'] as bool?,
    );

Map<String, dynamic> _$RepricingComparisonToJson(
  RepricingComparison instance,
) => <String, dynamic>{
  'difference': instance.difference,
  'lines': instance.lines,
  'new_price_date': instance.newPriceDate.toIso8601String(),
  'new_subtotal': instance.newSubtotal,
  'old_price_date': instance.oldPriceDate.toIso8601String(),
  'old_shipping_total': ?instance.oldShippingTotal,
  'old_subtotal': instance.oldSubtotal,
  'shipping_requote_needed': ?instance.shippingRequoteNeeded,
};
