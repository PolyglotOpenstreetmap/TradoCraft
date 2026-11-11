// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'portal_ticket.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

PortalTicket _$PortalTicketFromJson(Map<String, dynamic> json) => PortalTicket(
  category: json['category'] as String,
  contactName: json['contact_name'] as String,
  createdAt: DateTime.parse(json['created_at'] as String),
  id: json['id'] as String,
  number: json['number'] as String,
  status: json['status'] as String,
  subject: json['subject'] as String,
  updatedAt: DateTime.parse(json['updated_at'] as String),
  closedAt: json['closed_at'] == null
      ? null
      : DateTime.parse(json['closed_at'] as String),
);

Map<String, dynamic> _$PortalTicketToJson(PortalTicket instance) =>
    <String, dynamic>{
      'category': instance.category,
      'closed_at': ?instance.closedAt?.toIso8601String(),
      'contact_name': instance.contactName,
      'created_at': instance.createdAt.toIso8601String(),
      'id': instance.id,
      'number': instance.number,
      'status': instance.status,
      'subject': instance.subject,
      'updated_at': instance.updatedAt.toIso8601String(),
    };
