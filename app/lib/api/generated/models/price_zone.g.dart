// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'price_zone.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

PriceZone _$PriceZoneFromJson(Map<String, dynamic> json) => PriceZone(
  code: json['code'] as String,
  id: json['id'] as String,
  name: json['name'] as String,
  sortOrder: (json['sort_order'] as num).toInt(),
);

Map<String, dynamic> _$PriceZoneToJson(PriceZone instance) => <String, dynamic>{
  'code': instance.code,
  'id': instance.id,
  'name': instance.name,
  'sort_order': instance.sortOrder,
};
