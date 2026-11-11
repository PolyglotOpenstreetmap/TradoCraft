// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'contract_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ContractInput _$ContractInputFromJson(Map<String, dynamic> json) =>
    ContractInput(
      customerId: json['customer_id'] as String,
      startDate: DateTime.parse(json['start_date'] as String),
      title: json['title'] as String,
      autoRenew: json['auto_renew'] as bool?,
      city: json['city'] as String?,
      contactId: json['contact_id'] as String?,
      countryCode: json['country_code'] as String?,
      endDate: json['end_date'] == null
          ? null
          : DateTime.parse(json['end_date'] as String),
      equipmentIds: (json['equipment_ids'] as List<dynamic>?)
          ?.map((e) => e as String)
          .toList(),
      notes: json['notes'] as String?,
      postalCode: json['postal_code'] as String?,
      resolutionHours: (json['resolution_hours'] as num?)?.toInt(),
      responseHours: (json['response_hours'] as num?)?.toInt(),
      siteName: json['site_name'] as String?,
      street: json['street'] as String?,
      visitTemplates: (json['visit_templates'] as List<dynamic>?)
          ?.map((e) => TemplateInput.fromJson(e as Map<String, dynamic>))
          .toList(),
    );

Map<String, dynamic> _$ContractInputToJson(ContractInput instance) =>
    <String, dynamic>{
      'auto_renew': ?instance.autoRenew,
      'city': ?instance.city,
      'contact_id': ?instance.contactId,
      'country_code': ?instance.countryCode,
      'customer_id': instance.customerId,
      'end_date': ?instance.endDate?.toIso8601String(),
      'equipment_ids': ?instance.equipmentIds,
      'notes': ?instance.notes,
      'postal_code': ?instance.postalCode,
      'resolution_hours': ?instance.resolutionHours,
      'response_hours': ?instance.responseHours,
      'site_name': ?instance.siteName,
      'start_date': instance.startDate.toIso8601String(),
      'street': ?instance.street,
      'title': instance.title,
      'visit_templates': ?instance.visitTemplates,
    };
