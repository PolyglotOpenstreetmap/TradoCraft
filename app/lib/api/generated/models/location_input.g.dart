// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'location_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

LocationInput _$LocationInputFromJson(Map<String, dynamic> json) =>
    LocationInput(
      code: json['code'] as String,
      active: json['active'] as bool?,
      name: json['name'] as String?,
      warehouseId: json['warehouse_id'] as String?,
    );

Map<String, dynamic> _$LocationInputToJson(LocationInput instance) =>
    <String, dynamic>{
      'active': ?instance.active,
      'code': instance.code,
      'name': ?instance.name,
      'warehouse_id': ?instance.warehouseId,
    };
