// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'new_ticket.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

NewTicket _$NewTicketFromJson(Map<String, dynamic> json) => NewTicket(
  body: json['body'] as String,
  subject: json['subject'] as String,
  category: json['category'] as String?,
);

Map<String, dynamic> _$NewTicketToJson(NewTicket instance) => <String, dynamic>{
  'body': instance.body,
  'category': ?instance.category,
  'subject': instance.subject,
};
