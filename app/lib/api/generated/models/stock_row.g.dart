// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'stock_row.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

StockRow _$StockRowFromJson(Map<String, dynamic> json) => StockRow(
  available: json['available'] as String,
  location: json['location'] as String,
  locationId: json['location_id'] as String,
  onHand: json['on_hand'] as String,
  productCode: json['product_code'] as String,
  productId: json['product_id'] as String,
  productName: json['product_name'] as String,
  reserved: json['reserved'] as String,
  tracking: json['tracking'] as String,
  unit: json['unit'] as String,
  warehouseId: json['warehouse_id'] as String,
  warehouseKind: json['warehouse_kind'] as String,
  expiryDate: json['expiry_date'] == null
      ? null
      : DateTime.parse(json['expiry_date'] as String),
  lotId: json['lot_id'] as String?,
  lotNumber: json['lot_number'] as String?,
);

Map<String, dynamic> _$StockRowToJson(StockRow instance) => <String, dynamic>{
  'available': instance.available,
  'expiry_date': ?instance.expiryDate?.toIso8601String(),
  'location': instance.location,
  'location_id': instance.locationId,
  'lot_id': ?instance.lotId,
  'lot_number': ?instance.lotNumber,
  'on_hand': instance.onHand,
  'product_code': instance.productCode,
  'product_id': instance.productId,
  'product_name': instance.productName,
  'reserved': instance.reserved,
  'tracking': instance.tracking,
  'unit': instance.unit,
  'warehouse_id': instance.warehouseId,
  'warehouse_kind': instance.warehouseKind,
};
