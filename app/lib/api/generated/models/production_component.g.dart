// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'production_component.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ProductionComponent _$ProductionComponentFromJson(Map<String, dynamic> json) =>
    ProductionComponent(
      componentCode: json['component_code'] as String,
      componentName: json['component_name'] as String,
      componentProductId: json['component_product_id'] as String,
      consumedQuantity: json['consumed_quantity'] as String,
      id: json['id'] as String,
      manufactured: json['manufactured'] as bool,
      position: (json['position'] as num).toInt(),
      requiredQuantity: json['required_quantity'] as String,
      reservedQuantity: json['reserved_quantity'] as String,
      scrapQuantity: json['scrap_quantity'] as String,
      short: json['short'] as bool,
      tracking: json['tracking'] as String,
      unit: json['unit'] as String,
    );

Map<String, dynamic> _$ProductionComponentToJson(
  ProductionComponent instance,
) => <String, dynamic>{
  'component_code': instance.componentCode,
  'component_name': instance.componentName,
  'component_product_id': instance.componentProductId,
  'consumed_quantity': instance.consumedQuantity,
  'id': instance.id,
  'manufactured': instance.manufactured,
  'position': instance.position,
  'required_quantity': instance.requiredQuantity,
  'reserved_quantity': instance.reservedQuantity,
  'scrap_quantity': instance.scrapQuantity,
  'short': instance.short,
  'tracking': instance.tracking,
  'unit': instance.unit,
};
