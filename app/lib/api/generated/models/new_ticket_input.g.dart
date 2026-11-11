// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'new_ticket_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

NewTicketInput _$NewTicketInputFromJson(Map<String, dynamic> json) =>
    NewTicketInput(
      body: json['body'] as String,
      subject: json['subject'] as String,
      category: json['category'] as String?,
      contactEmail: json['contact_email'] as String?,
      contactId: json['contact_id'] as String?,
      contactName: json['contact_name'] as String?,
      customerId: json['customer_id'] as String?,
      internal: json['internal'] as bool?,
      language: json['language'] as String?,
    );

Map<String, dynamic> _$NewTicketInputToJson(NewTicketInput instance) =>
    <String, dynamic>{
      'body': instance.body,
      'category': ?instance.category,
      'contact_email': ?instance.contactEmail,
      'contact_id': ?instance.contactId,
      'contact_name': ?instance.contactName,
      'customer_id': ?instance.customerId,
      'internal': ?instance.internal,
      'language': ?instance.language,
      'subject': instance.subject,
    };
