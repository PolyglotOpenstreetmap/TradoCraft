// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'contract_detail.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ContractDetail _$ContractDetailFromJson(Map<String, dynamic> json) =>
    ContractDetail(
      autoRenew: json['auto_renew'] as bool,
      customerId: json['customer_id'] as String,
      customerName: json['customer_name'] as String,
      id: json['id'] as String,
      number: json['number'] as String,
      openJobs: (json['open_jobs'] as num).toInt(),
      startDate: DateTime.parse(json['start_date'] as String),
      status: json['status'] as String,
      title: json['title'] as String,
      city: json['city'] as String,
      countryCode: json['country_code'] as String,
      createdAt: DateTime.parse(json['created_at'] as String),
      equipment: (json['equipment'] as List<dynamic>)
          .map((e) => JobEquipment.fromJson(e as Map<String, dynamic>))
          .toList(),
      notes: json['notes'] as String,
      postalCode: json['postal_code'] as String,
      siteName: json['site_name'] as String,
      street: json['street'] as String,
      visitTemplates: (json['visit_templates'] as List<dynamic>)
          .map((e) => VisitTemplate.fromJson(e as Map<String, dynamic>))
          .toList(),
      endDate: json['end_date'] == null
          ? null
          : DateTime.parse(json['end_date'] as String),
      resolutionHours: (json['resolution_hours'] as num?)?.toInt(),
      responseHours: (json['response_hours'] as num?)?.toInt(),
      contactId: json['contact_id'] as String?,
      contactName: json['contact_name'] as String?,
    );

Map<String, dynamic> _$ContractDetailToJson(ContractDetail instance) =>
    <String, dynamic>{
      'auto_renew': instance.autoRenew,
      'customer_id': instance.customerId,
      'customer_name': instance.customerName,
      'end_date': ?instance.endDate?.toIso8601String(),
      'id': instance.id,
      'number': instance.number,
      'open_jobs': instance.openJobs,
      'resolution_hours': ?instance.resolutionHours,
      'response_hours': ?instance.responseHours,
      'start_date': instance.startDate.toIso8601String(),
      'status': instance.status,
      'title': instance.title,
      'city': instance.city,
      'contact_id': ?instance.contactId,
      'contact_name': ?instance.contactName,
      'country_code': instance.countryCode,
      'created_at': instance.createdAt.toIso8601String(),
      'equipment': instance.equipment,
      'notes': instance.notes,
      'postal_code': instance.postalCode,
      'site_name': instance.siteName,
      'street': instance.street,
      'visit_templates': instance.visitTemplates,
    };
