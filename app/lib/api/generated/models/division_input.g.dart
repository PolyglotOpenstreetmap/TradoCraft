// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'division_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

DivisionInput _$DivisionInputFromJson(Map<String, dynamic> json) =>
    DivisionInput(
      code: json['code'] as String,
      name: json['name'] as String,
      active: json['active'] as bool?,
      sortOrder: (json['sort_order'] as num?)?.toInt(),
    );

Map<String, dynamic> _$DivisionInputToJson(DivisionInput instance) =>
    <String, dynamic>{
      'active': ?instance.active,
      'code': instance.code,
      'name': instance.name,
      'sort_order': ?instance.sortOrder,
    };
