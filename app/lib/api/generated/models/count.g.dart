// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'count.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Count _$CountFromJson(Map<String, dynamic> json) => Count(
  countedCount: (json['counted_count'] as num).toInt(),
  createdAt: DateTime.parse(json['created_at'] as String),
  createdByName: json['created_by_name'] as String,
  id: json['id'] as String,
  lineCount: (json['line_count'] as num).toInt(),
  note: json['note'] as String,
  number: json['number'] as String,
  status: json['status'] as String,
  warehouseCode: json['warehouse_code'] as String,
  warehouseId: json['warehouse_id'] as String,
  closedAt: json['closed_at'] == null
      ? null
      : DateTime.parse(json['closed_at'] as String),
  locationCode: json['location_code'] as String?,
  locationId: json['location_id'] as String?,
  productId: json['product_id'] as String?,
);

Map<String, dynamic> _$CountToJson(Count instance) => <String, dynamic>{
  'closed_at': ?instance.closedAt?.toIso8601String(),
  'counted_count': instance.countedCount,
  'created_at': instance.createdAt.toIso8601String(),
  'created_by_name': instance.createdByName,
  'id': instance.id,
  'line_count': instance.lineCount,
  'location_code': ?instance.locationCode,
  'location_id': ?instance.locationId,
  'note': instance.note,
  'number': instance.number,
  'product_id': ?instance.productId,
  'status': instance.status,
  'warehouse_code': instance.warehouseCode,
  'warehouse_id': instance.warehouseId,
};
