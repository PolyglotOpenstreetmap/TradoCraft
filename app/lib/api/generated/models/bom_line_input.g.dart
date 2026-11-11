// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'bom_line_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

BomLineInput _$BomLineInputFromJson(Map<String, dynamic> json) => BomLineInput(
  componentProductId: json['component_product_id'] as String,
  quantity: json['quantity'] as String,
  notes: json['notes'] as String?,
  scrapPercent: json['scrap_percent'] as String?,
);

Map<String, dynamic> _$BomLineInputToJson(BomLineInput instance) =>
    <String, dynamic>{
      'component_product_id': instance.componentProductId,
      'notes': ?instance.notes,
      'quantity': instance.quantity,
      'scrap_percent': ?instance.scrapPercent,
    };
