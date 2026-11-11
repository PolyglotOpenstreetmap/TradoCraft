// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'count_line.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

CountLine _$CountLineFromJson(Map<String, dynamic> json) => CountLine(
  expectedQuantity: json['expected_quantity'] as String,
  id: json['id'] as String,
  location: json['location'] as String,
  locationId: json['location_id'] as String,
  productCode: json['product_code'] as String,
  productId: json['product_id'] as String,
  productName: json['product_name'] as String,
  unit: json['unit'] as String,
  adjustmentMoveId: json['adjustment_move_id'] as String?,
  countedQuantity: json['counted_quantity'] as String?,
  differenceToExpected: json['difference_to_expected'] as String?,
  lotId: json['lot_id'] as String?,
  lotNumber: json['lot_number'] as String?,
  quantityAtPost: json['quantity_at_post'] as String?,
);

Map<String, dynamic> _$CountLineToJson(CountLine instance) => <String, dynamic>{
  'adjustment_move_id': ?instance.adjustmentMoveId,
  'counted_quantity': ?instance.countedQuantity,
  'difference_to_expected': ?instance.differenceToExpected,
  'expected_quantity': instance.expectedQuantity,
  'id': instance.id,
  'location': instance.location,
  'location_id': instance.locationId,
  'lot_id': ?instance.lotId,
  'lot_number': ?instance.lotNumber,
  'product_code': instance.productCode,
  'product_id': instance.productId,
  'product_name': instance.productName,
  'quantity_at_post': ?instance.quantityAtPost,
  'unit': instance.unit,
};
