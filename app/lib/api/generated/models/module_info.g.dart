// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'module_info.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ModuleInfo _$ModuleInfoFromJson(Map<String, dynamic> json) => ModuleInfo(
  code: json['code'] as String,
  enabled: json['enabled'] as bool,
  permissions: (json['permissions'] as List<dynamic>)
      .map((e) => e as String)
      .toList(),
  requires: (json['requires'] as List<dynamic>)
      .map((e) => e as String)
      .toList(),
);

Map<String, dynamic> _$ModuleInfoToJson(ModuleInfo instance) =>
    <String, dynamic>{
      'code': instance.code,
      'enabled': instance.enabled,
      'permissions': instance.permissions,
      'requires': instance.requires,
    };
