// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'me.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Me _$MeFromJson(Map<String, dynamic> json) => Me(
  company: Company.fromJson(json['company'] as Map<String, dynamic>),
  displayName: json['display_name'] as String,
  email: json['email'] as String,
  memberships: (json['memberships'] as List<dynamic>)
      .map((e) => Membership.fromJson(e as Map<String, dynamic>))
      .toList(),
  modules: (json['modules'] as List<dynamic>)
      .map((e) => MeModule.fromJson(e as Map<String, dynamic>))
      .toList(),
  permissions: (json['permissions'] as List<dynamic>)
      .map((e) => e as String)
      .toList(),
  roles: (json['roles'] as List<dynamic>).map((e) => e as String).toList(),
  userId: json['user_id'] as String,
);

Map<String, dynamic> _$MeToJson(Me instance) => <String, dynamic>{
  'company': instance.company,
  'display_name': instance.displayName,
  'email': instance.email,
  'memberships': instance.memberships,
  'modules': instance.modules,
  'permissions': instance.permissions,
  'roles': instance.roles,
  'user_id': instance.userId,
};
