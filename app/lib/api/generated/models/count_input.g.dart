// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'count_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

CountInput _$CountInputFromJson(Map<String, dynamic> json) => CountInput(
  warehouseId: json['warehouse_id'] as String,
  locationId: json['location_id'] as String?,
  note: json['note'] as String?,
  productId: json['product_id'] as String?,
);

Map<String, dynamic> _$CountInputToJson(CountInput instance) =>
    <String, dynamic>{
      'location_id': ?instance.locationId,
      'note': ?instance.note,
      'product_id': ?instance.productId,
      'warehouse_id': instance.warehouseId,
    };
