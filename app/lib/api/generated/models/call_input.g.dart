// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'call_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

CallInput _$CallInputFromJson(Map<String, dynamic> json) => CallInput(
  direction: json['direction'] as String,
  summary: json['summary'] as String,
  category: json['category'] as String?,
  contactId: json['contact_id'] as String?,
  customerId: json['customer_id'] as String?,
  durationSeconds: (json['duration_seconds'] as num?)?.toInt(),
  phoneNumber: json['phone_number'] as String?,
  subject: json['subject'] as String?,
  ticketId: json['ticket_id'] as String?,
);

Map<String, dynamic> _$CallInputToJson(CallInput instance) => <String, dynamic>{
  'category': ?instance.category,
  'contact_id': ?instance.contactId,
  'customer_id': ?instance.customerId,
  'direction': instance.direction,
  'duration_seconds': ?instance.durationSeconds,
  'phone_number': ?instance.phoneNumber,
  'subject': ?instance.subject,
  'summary': instance.summary,
  'ticket_id': ?instance.ticketId,
};
