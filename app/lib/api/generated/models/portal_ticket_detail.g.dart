// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'portal_ticket_detail.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

PortalTicketDetail _$PortalTicketDetailFromJson(Map<String, dynamic> json) =>
    PortalTicketDetail(
      messages: (json['messages'] as List<dynamic>)
          .map((e) => PortalMessage.fromJson(e as Map<String, dynamic>))
          .toList(),
      ticket: PortalTicket.fromJson(json['ticket'] as Map<String, dynamic>),
    );

Map<String, dynamic> _$PortalTicketDetailToJson(PortalTicketDetail instance) =>
    <String, dynamic>{'messages': instance.messages, 'ticket': instance.ticket};
