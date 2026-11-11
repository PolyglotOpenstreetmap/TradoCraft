// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'department.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Department _$DepartmentFromJson(Map<String, dynamic> json) => Department(
  active: json['active'] as bool,
  code: json['code'] as String,
  id: json['id'] as String,
  members: (json['members'] as List<dynamic>)
      .map((e) => Member.fromJson(e as Map<String, dynamic>))
      .toList(),
  name: json['name'] as String,
  openRepairs: (json['open_repairs'] as num).toInt(),
  sortOrder: (json['sort_order'] as num).toInt(),
);

Map<String, dynamic> _$DepartmentToJson(Department instance) =>
    <String, dynamic>{
      'active': instance.active,
      'code': instance.code,
      'id': instance.id,
      'members': instance.members,
      'name': instance.name,
      'open_repairs': instance.openRepairs,
      'sort_order': instance.sortOrder,
    };
