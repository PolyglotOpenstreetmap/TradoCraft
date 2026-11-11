// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'admin_user.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

AdminUser _$AdminUserFromJson(Map<String, dynamic> json) => AdminUser(
  active: json['active'] as bool,
  disabledModules: (json['disabled_modules'] as List<dynamic>)
      .map((e) => e as String)
      .toList(),
  displayName: json['display_name'] as String,
  email: json['email'] as String,
  hasSignedIn: json['has_signed_in'] as bool,
  roles: (json['roles'] as List<dynamic>).map((e) => e as String).toList(),
  userId: json['user_id'] as String,
);

Map<String, dynamic> _$AdminUserToJson(AdminUser instance) => <String, dynamic>{
  'active': instance.active,
  'disabled_modules': instance.disabledModules,
  'display_name': instance.displayName,
  'email': instance.email,
  'has_signed_in': instance.hasSignedIn,
  'roles': instance.roles,
  'user_id': instance.userId,
};
