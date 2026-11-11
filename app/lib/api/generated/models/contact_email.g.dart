// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'contact_email.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ContactEmail _$ContactEmailFromJson(Map<String, dynamic> json) => ContactEmail(
  email: json['email'] as String,
  isPrimary: json['is_primary'] as bool,
);

Map<String, dynamic> _$ContactEmailToJson(ContactEmail instance) =>
    <String, dynamic>{
      'email': instance.email,
      'is_primary': instance.isPrimary,
    };
