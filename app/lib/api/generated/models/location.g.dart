// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'location.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Location _$LocationFromJson(Map<String, dynamic> json) => Location(
  active: json['active'] as bool,
  code: json['code'] as String,
  id: json['id'] as String,
  name: json['name'] as String,
  warehouseId: json['warehouse_id'] as String,
);

Map<String, dynamic> _$LocationToJson(Location instance) => <String, dynamic>{
  'active': instance.active,
  'code': instance.code,
  'id': instance.id,
  'name': instance.name,
  'warehouse_id': instance.warehouseId,
};
