// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'production_component_actual.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ProductionComponentActual _$ProductionComponentActualFromJson(
  Map<String, dynamic> json,
) => ProductionComponentActual(
  componentId: json['component_id'] as String,
  consumedQuantity: json['consumed_quantity'] as String?,
  extraLocationId: json['extra_location_id'] as String?,
  extraLotId: json['extra_lot_id'] as String?,
  scrapQuantity: json['scrap_quantity'] as String?,
  serialUnitIds: (json['serial_unit_ids'] as List<dynamic>?)
      ?.map((e) => e as String)
      .toList(),
);

Map<String, dynamic> _$ProductionComponentActualToJson(
  ProductionComponentActual instance,
) => <String, dynamic>{
  'component_id': instance.componentId,
  'consumed_quantity': ?instance.consumedQuantity,
  'extra_location_id': ?instance.extraLocationId,
  'extra_lot_id': ?instance.extraLotId,
  'scrap_quantity': ?instance.scrapQuantity,
  'serial_unit_ids': ?instance.serialUnitIds,
};
