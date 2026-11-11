// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'job.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Job _$JobFromJson(Map<String, dynamic> json) => Job(
  street: json['street'] as String,
  city: json['city'] as String,
  completionNotes: json['completion_notes'] as String,
  id: json['id'] as String,
  countryCode: json['country_code'] as String,
  customerId: json['customer_id'] as String,
  customerName: json['customer_name'] as String,
  address: json['address'] as String,
  instructions: json['instructions'] as String,
  kind: json['kind'] as String,
  number: json['number'] as String,
  postalCode: json['postal_code'] as String,
  status: json['status'] as String,
  tasksTotal: (json['tasks_total'] as num).toInt(),
  technicians: (json['technicians'] as List<dynamic>)
      .map((e) => Technician.fromJson(e as Map<String, dynamic>))
      .toList(),
  reportSentTo: json['report_sent_to'] as String,
  title: json['title'] as String,
  priority: json['priority'] as String,
  tasksDone: (json['tasks_done'] as num).toInt(),
  siteName: json['site_name'] as String,
  contactName: json['contact_name'] as String?,
  repairId: json['repair_id'] as String?,
  longitude: (json['longitude'] as num?)?.toDouble(),
  latitude: (json['latitude'] as num?)?.toDouble(),
  scheduledEnd: json['scheduled_end'] == null
      ? null
      : DateTime.parse(json['scheduled_end'] as String),
  scheduledStart: json['scheduled_start'] == null
      ? null
      : DateTime.parse(json['scheduled_start'] as String),
  dueDate: json['due_date'] == null
      ? null
      : DateTime.parse(json['due_date'] as String),
  startedAt: json['started_at'] == null
      ? null
      : DateTime.parse(json['started_at'] as String),
  contractNumber: json['contract_number'] as String?,
  reportSentAt: json['report_sent_at'] == null
      ? null
      : DateTime.parse(json['report_sent_at'] as String),
  contractId: json['contract_id'] as String?,
  contactId: json['contact_id'] as String?,
  completedAt: json['completed_at'] == null
      ? null
      : DateTime.parse(json['completed_at'] as String),
  ticketId: json['ticket_id'] as String?,
  mapsUrl: json['maps_url'] as String?,
);

Map<String, dynamic> _$JobToJson(Job instance) => <String, dynamic>{
  'address': instance.address,
  'city': instance.city,
  'completed_at': ?instance.completedAt?.toIso8601String(),
  'completion_notes': instance.completionNotes,
  'contact_id': ?instance.contactId,
  'contact_name': ?instance.contactName,
  'contract_id': ?instance.contractId,
  'contract_number': ?instance.contractNumber,
  'country_code': instance.countryCode,
  'customer_id': instance.customerId,
  'customer_name': instance.customerName,
  'due_date': ?instance.dueDate?.toIso8601String(),
  'id': instance.id,
  'instructions': instance.instructions,
  'kind': instance.kind,
  'latitude': ?instance.latitude,
  'longitude': ?instance.longitude,
  'maps_url': ?instance.mapsUrl,
  'number': instance.number,
  'postal_code': instance.postalCode,
  'priority': instance.priority,
  'repair_id': ?instance.repairId,
  'report_sent_at': ?instance.reportSentAt?.toIso8601String(),
  'report_sent_to': instance.reportSentTo,
  'scheduled_end': ?instance.scheduledEnd?.toIso8601String(),
  'scheduled_start': ?instance.scheduledStart?.toIso8601String(),
  'site_name': instance.siteName,
  'started_at': ?instance.startedAt?.toIso8601String(),
  'status': instance.status,
  'street': instance.street,
  'tasks_done': instance.tasksDone,
  'tasks_total': instance.tasksTotal,
  'technicians': instance.technicians,
  'ticket_id': ?instance.ticketId,
  'title': instance.title,
};
