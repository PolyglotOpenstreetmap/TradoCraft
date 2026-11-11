// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'role_info.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

RoleInfo _$RoleInfoFromJson(Map<String, dynamic> json) => RoleInfo(
  permissions: (json['permissions'] as List<dynamic>)
      .map((e) => e as String)
      .toList(),
  role: json['role'] as String,
);

Map<String, dynamic> _$RoleInfoToJson(RoleInfo instance) => <String, dynamic>{
  'permissions': instance.permissions,
  'role': instance.role,
};
