// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'warehouse.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Warehouse _$WarehouseFromJson(Map<String, dynamic> json) => Warehouse(
  active: json['active'] as bool,
  code: json['code'] as String,
  id: json['id'] as String,
  kind: json['kind'] as String,
  locations: (json['locations'] as List<dynamic>)
      .map((e) => Location.fromJson(e as Map<String, dynamic>))
      .toList(),
  name: json['name'] as String,
  assignedUserId: json['assigned_user_id'] as String?,
  assignedUserName: json['assigned_user_name'] as String?,
);

Map<String, dynamic> _$WarehouseToJson(Warehouse instance) => <String, dynamic>{
  'active': instance.active,
  'assigned_user_id': ?instance.assignedUserId,
  'assigned_user_name': ?instance.assignedUserName,
  'code': instance.code,
  'id': instance.id,
  'kind': instance.kind,
  'locations': instance.locations,
  'name': instance.name,
};
