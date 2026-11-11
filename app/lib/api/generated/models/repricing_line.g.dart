// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'repricing_line.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

RepricingLine _$RepricingLineFromJson(Map<String, dynamic> json) =>
    RepricingLine(
      description: json['description'] as String,
      difference: json['difference'] as String,
      needsConfirmation: json['needs_confirmation'] as bool,
      newLineTotal: json['new_line_total'] as String,
      newListUnitPrice: json['new_list_unit_price'] as String,
      newRebatePercent: json['new_rebate_percent'] as String,
      oldLineTotal: json['old_line_total'] as String,
      oldListUnitPrice: json['old_list_unit_price'] as String,
      oldRebatePercent: json['old_rebate_percent'] as String,
      position: (json['position'] as num).toInt(),
      pricingType: json['pricing_type'] as String,
      productCode: json['product_code'] as String,
      quantity: json['quantity'] as String,
    );

Map<String, dynamic> _$RepricingLineToJson(RepricingLine instance) =>
    <String, dynamic>{
      'description': instance.description,
      'difference': instance.difference,
      'needs_confirmation': instance.needsConfirmation,
      'new_line_total': instance.newLineTotal,
      'new_list_unit_price': instance.newListUnitPrice,
      'new_rebate_percent': instance.newRebatePercent,
      'old_line_total': instance.oldLineTotal,
      'old_list_unit_price': instance.oldListUnitPrice,
      'old_rebate_percent': instance.oldRebatePercent,
      'position': instance.position,
      'pricing_type': instance.pricingType,
      'product_code': instance.productCode,
      'quantity': instance.quantity,
    };
