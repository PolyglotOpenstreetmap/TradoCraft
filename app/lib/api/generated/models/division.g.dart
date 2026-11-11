// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'division.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Division _$DivisionFromJson(Map<String, dynamic> json) => Division(
  active: json['active'] as bool,
  code: json['code'] as String,
  id: json['id'] as String,
  name: json['name'] as String,
  openOrders: (json['open_orders'] as num).toInt(),
  sortOrder: (json['sort_order'] as num).toInt(),
);

Map<String, dynamic> _$DivisionToJson(Division instance) => <String, dynamic>{
  'active': instance.active,
  'code': instance.code,
  'id': instance.id,
  'name': instance.name,
  'open_orders': instance.openOrders,
  'sort_order': instance.sortOrder,
};
