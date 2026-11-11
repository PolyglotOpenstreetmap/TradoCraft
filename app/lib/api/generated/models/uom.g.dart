// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'uom.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Uom _$UomFromJson(Map<String, dynamic> json) => Uom(
  active: json['active'] as bool,
  category: json['category'] as String,
  code: json['code'] as String,
  factor: json['factor'] as String,
  name: json['name'] as String,
);

Map<String, dynamic> _$UomToJson(Uom instance) => <String, dynamic>{
  'active': instance.active,
  'category': instance.category,
  'code': instance.code,
  'factor': instance.factor,
  'name': instance.name,
};
