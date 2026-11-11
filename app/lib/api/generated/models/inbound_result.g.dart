// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'inbound_result.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

InboundResult _$InboundResultFromJson(Map<String, dynamic> json) =>
    InboundResult(
      duplicate: json['duplicate'] as bool,
      newTicket: json['new_ticket'] as bool,
      ticketId: json['ticket_id'] as String,
      ticketNumber: json['ticket_number'] as String,
    );

Map<String, dynamic> _$InboundResultToJson(InboundResult instance) =>
    <String, dynamic>{
      'duplicate': instance.duplicate,
      'new_ticket': instance.newTicket,
      'ticket_id': instance.ticketId,
      'ticket_number': instance.ticketNumber,
    };
