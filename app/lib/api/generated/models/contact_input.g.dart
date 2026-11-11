// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'contact_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ContactInput _$ContactInputFromJson(Map<String, dynamic> json) => ContactInput(
  lastName: json['last_name'] as String,
  active: json['active'] as bool?,
  addressForm: json['address_form'] as String?,
  emails: (json['emails'] as List<dynamic>?)?.map((e) => e as String).toList(),
  firstName: json['first_name'] as String?,
  jobTitle: json['job_title'] as String?,
  language: json['language'] as String?,
  notes: json['notes'] as String?,
  phones: (json['phones'] as List<dynamic>?)
      ?.map((e) => PhoneInput.fromJson(e as Map<String, dynamic>))
      .toList(),
  portalAccess: json['portal_access'] as bool?,
  portalScope: json['portal_scope'] as String?,
  roles: (json['roles'] as List<dynamic>?)?.map((e) => e as String).toList(),
  title: json['title'] as String?,
);

Map<String, dynamic> _$ContactInputToJson(ContactInput instance) =>
    <String, dynamic>{
      'active': ?instance.active,
      'address_form': ?instance.addressForm,
      'emails': ?instance.emails,
      'first_name': ?instance.firstName,
      'job_title': ?instance.jobTitle,
      'language': ?instance.language,
      'last_name': instance.lastName,
      'notes': ?instance.notes,
      'phones': ?instance.phones,
      'portal_access': ?instance.portalAccess,
      'portal_scope': ?instance.portalScope,
      'roles': ?instance.roles,
      'title': ?instance.title,
    };
