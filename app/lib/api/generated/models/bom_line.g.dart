// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'bom_line.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

BomLine _$BomLineFromJson(Map<String, dynamic> json) => BomLine(
  componentCode: json['component_code'] as String,
  componentHasBom: json['component_has_bom'] as bool,
  componentName: json['component_name'] as String,
  componentProductId: json['component_product_id'] as String,
  componentTracking: json['component_tracking'] as String,
  componentType: json['component_type'] as String,
  componentUnit: json['component_unit'] as String,
  notes: json['notes'] as String,
  position: (json['position'] as num).toInt(),
  quantity: json['quantity'] as String,
  scrapPercent: json['scrap_percent'] as String,
);

Map<String, dynamic> _$BomLineToJson(BomLine instance) => <String, dynamic>{
  'component_code': instance.componentCode,
  'component_has_bom': instance.componentHasBom,
  'component_name': instance.componentName,
  'component_product_id': instance.componentProductId,
  'component_tracking': instance.componentTracking,
  'component_type': instance.componentType,
  'component_unit': instance.componentUnit,
  'notes': instance.notes,
  'position': instance.position,
  'quantity': instance.quantity,
  'scrap_percent': instance.scrapPercent,
};
