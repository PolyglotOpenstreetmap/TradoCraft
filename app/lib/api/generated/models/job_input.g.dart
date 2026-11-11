// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'job_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

JobInput _$JobInputFromJson(Map<String, dynamic> json) => JobInput(
  customerId: json['customer_id'] as String,
  title: json['title'] as String,
  city: json['city'] as String?,
  contactId: json['contact_id'] as String?,
  countryCode: json['country_code'] as String?,
  instructions: json['instructions'] as String?,
  kind: json['kind'] as String?,
  latitude: (json['latitude'] as num?)?.toDouble(),
  longitude: (json['longitude'] as num?)?.toDouble(),
  postalCode: json['postal_code'] as String?,
  priority: json['priority'] as String?,
  repairId: json['repair_id'] as String?,
  scheduledEnd: json['scheduled_end'] == null
      ? null
      : DateTime.parse(json['scheduled_end'] as String),
  scheduledStart: json['scheduled_start'] == null
      ? null
      : DateTime.parse(json['scheduled_start'] as String),
  siteName: json['site_name'] as String?,
  street: json['street'] as String?,
  tasks: (json['tasks'] as List<dynamic>?)?.map((e) => e as String).toList(),
  technicianIds: (json['technician_ids'] as List<dynamic>?)
      ?.map((e) => e as String)
      .toList(),
  ticketId: json['ticket_id'] as String?,
);

Map<String, dynamic> _$JobInputToJson(JobInput instance) => <String, dynamic>{
  'city': ?instance.city,
  'contact_id': ?instance.contactId,
  'country_code': ?instance.countryCode,
  'customer_id': instance.customerId,
  'instructions': ?instance.instructions,
  'kind': ?instance.kind,
  'latitude': ?instance.latitude,
  'longitude': ?instance.longitude,
  'postal_code': ?instance.postalCode,
  'priority': ?instance.priority,
  'repair_id': ?instance.repairId,
  'scheduled_end': ?instance.scheduledEnd?.toIso8601String(),
  'scheduled_start': ?instance.scheduledStart?.toIso8601String(),
  'site_name': ?instance.siteName,
  'street': ?instance.street,
  'tasks': ?instance.tasks,
  'technician_ids': ?instance.technicianIds,
  'ticket_id': ?instance.ticketId,
  'title': instance.title,
};
