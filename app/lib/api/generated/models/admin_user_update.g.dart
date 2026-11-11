// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'admin_user_update.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

AdminUserUpdate _$AdminUserUpdateFromJson(Map<String, dynamic> json) =>
    AdminUserUpdate(
      active: json['active'] as bool,
      disabledModules: (json['disabled_modules'] as List<dynamic>)
          .map((e) => e as String)
          .toList(),
      displayName: json['display_name'] as String,
      roles: (json['roles'] as List<dynamic>).map((e) => e as String).toList(),
    );

Map<String, dynamic> _$AdminUserUpdateToJson(AdminUserUpdate instance) =>
    <String, dynamic>{
      'active': instance.active,
      'disabled_modules': instance.disabledModules,
      'display_name': instance.displayName,
      'roles': instance.roles,
    };
