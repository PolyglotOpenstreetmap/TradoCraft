// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'contact.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Contact _$ContactFromJson(Map<String, dynamic> json) => Contact(
  active: json['active'] as bool,
  addressForm: json['address_form'] as String,
  createdAt: DateTime.parse(json['created_at'] as String),
  customerCode: json['customer_code'] as String,
  customerId: json['customer_id'] as String,
  customerLanguage: json['customer_language'] as String,
  customerName: json['customer_name'] as String,
  effectiveLanguage: json['effective_language'] as String,
  emails: (json['emails'] as List<dynamic>)
      .map((e) => ContactEmail.fromJson(e as Map<String, dynamic>))
      .toList(),
  firstName: json['first_name'] as String,
  fullName: json['full_name'] as String,
  greeting: json['greeting'] as String,
  id: json['id'] as String,
  jobTitle: json['job_title'] as String,
  lastName: json['last_name'] as String,
  notes: json['notes'] as String,
  phones: (json['phones'] as List<dynamic>)
      .map((e) => ContactPhone.fromJson(e as Map<String, dynamic>))
      .toList(),
  portalAccess: json['portal_access'] as bool,
  portalScope: json['portal_scope'] as String,
  roles: (json['roles'] as List<dynamic>).map((e) => e as String).toList(),
  title: json['title'] as String,
  updatedAt: DateTime.parse(json['updated_at'] as String),
  language: json['language'] as String?,
);

Map<String, dynamic> _$ContactToJson(Contact instance) => <String, dynamic>{
  'active': instance.active,
  'address_form': instance.addressForm,
  'created_at': instance.createdAt.toIso8601String(),
  'customer_code': instance.customerCode,
  'customer_id': instance.customerId,
  'customer_language': instance.customerLanguage,
  'customer_name': instance.customerName,
  'effective_language': instance.effectiveLanguage,
  'emails': instance.emails,
  'first_name': instance.firstName,
  'full_name': instance.fullName,
  'greeting': instance.greeting,
  'id': instance.id,
  'job_title': instance.jobTitle,
  'language': ?instance.language,
  'last_name': instance.lastName,
  'notes': instance.notes,
  'phones': instance.phones,
  'portal_access': instance.portalAccess,
  'portal_scope': instance.portalScope,
  'roles': instance.roles,
  'title': instance.title,
  'updated_at': instance.updatedAt.toIso8601String(),
};
