// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'csat_info.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

CsatInfo _$CsatInfoFromJson(Map<String, dynamic> json) => CsatInfo(
  companyName: json['company_name'] as String,
  language: json['language'] as String,
  state: json['state'] as String,
  subject: json['subject'] as String,
  ticketNumber: json['ticket_number'] as String,
  rating: json['rating'] as String?,
);

Map<String, dynamic> _$CsatInfoToJson(CsatInfo instance) => <String, dynamic>{
  'company_name': instance.companyName,
  'language': instance.language,
  'rating': ?instance.rating,
  'state': instance.state,
  'subject': instance.subject,
  'ticket_number': instance.ticketNumber,
};
