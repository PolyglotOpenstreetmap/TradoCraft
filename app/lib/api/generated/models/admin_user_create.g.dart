// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'admin_user_create.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

AdminUserCreate _$AdminUserCreateFromJson(Map<String, dynamic> json) =>
    AdminUserCreate(
      displayName: json['display_name'] as String,
      email: json['email'] as String,
      roles: (json['roles'] as List<dynamic>).map((e) => e as String).toList(),
      disabledModules: (json['disabled_modules'] as List<dynamic>?)
          ?.map((e) => e as String)
          .toList(),
    );

Map<String, dynamic> _$AdminUserCreateToJson(AdminUserCreate instance) =>
    <String, dynamic>{
      'disabled_modules': ?instance.disabledModules,
      'display_name': instance.displayName,
      'email': instance.email,
      'roles': instance.roles,
    };
