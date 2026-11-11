// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'company_user.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

CompanyUser _$CompanyUserFromJson(Map<String, dynamic> json) => CompanyUser(
  displayName: json['display_name'] as String,
  email: json['email'] as String,
  id: json['id'] as String,
  roles: (json['roles'] as List<dynamic>).map((e) => e as String).toList(),
);

Map<String, dynamic> _$CompanyUserToJson(CompanyUser instance) =>
    <String, dynamic>{
      'display_name': instance.displayName,
      'email': instance.email,
      'id': instance.id,
      'roles': instance.roles,
    };
