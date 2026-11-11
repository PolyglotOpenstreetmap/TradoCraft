// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'warehouse_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

WarehouseInput _$WarehouseInputFromJson(Map<String, dynamic> json) =>
    WarehouseInput(
      code: json['code'] as String,
      name: json['name'] as String,
      active: json['active'] as bool?,
      assignedUserId: json['assigned_user_id'] as String?,
      kind: json['kind'] as String?,
    );

Map<String, dynamic> _$WarehouseInputToJson(WarehouseInput instance) =>
    <String, dynamic>{
      'active': ?instance.active,
      'assigned_user_id': ?instance.assignedUserId,
      'code': instance.code,
      'kind': ?instance.kind,
      'name': instance.name,
    };
