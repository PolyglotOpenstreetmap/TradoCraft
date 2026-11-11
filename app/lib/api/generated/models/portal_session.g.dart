// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'portal_session.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

PortalSession _$PortalSessionFromJson(Map<String, dynamic> json) =>
    PortalSession(
      expiresAt: DateTime.parse(json['expires_at'] as String),
      profile: PortalProfile.fromJson(json['profile'] as Map<String, dynamic>),
      token: json['token'] as String,
    );

Map<String, dynamic> _$PortalSessionToJson(PortalSession instance) =>
    <String, dynamic>{
      'expires_at': instance.expiresAt.toIso8601String(),
      'profile': instance.profile,
      'token': instance.token,
    };
