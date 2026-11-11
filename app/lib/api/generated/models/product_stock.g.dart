// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'product_stock.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ProductStock _$ProductStockFromJson(Map<String, dynamic> json) => ProductStock(
  available: json['available'] as String,
  belowReorder: json['below_reorder'] as bool,
  code: json['code'] as String,
  inVans: json['in_vans'] as String,
  name: json['name'] as String,
  onHand: json['on_hand'] as String,
  productId: json['product_id'] as String,
  reserved: json['reserved'] as String,
  tracking: json['tracking'] as String,
  unit: json['unit'] as String,
  barcode: json['barcode'] as String?,
  leadTimeDays: (json['lead_time_days'] as num?)?.toInt(),
  reorderMax: json['reorder_max'] as String?,
  reorderMin: json['reorder_min'] as String?,
);

Map<String, dynamic> _$ProductStockToJson(ProductStock instance) =>
    <String, dynamic>{
      'available': instance.available,
      'barcode': ?instance.barcode,
      'below_reorder': instance.belowReorder,
      'code': instance.code,
      'in_vans': instance.inVans,
      'lead_time_days': ?instance.leadTimeDays,
      'name': instance.name,
      'on_hand': instance.onHand,
      'product_id': instance.productId,
      'reorder_max': ?instance.reorderMax,
      'reorder_min': ?instance.reorderMin,
      'reserved': instance.reserved,
      'tracking': instance.tracking,
      'unit': instance.unit,
    };
