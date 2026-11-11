// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'ticket_page.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

TicketPage _$TicketPageFromJson(Map<String, dynamic> json) => TicketPage(
  items: (json['items'] as List<dynamic>)
      .map((e) => Ticket.fromJson(e as Map<String, dynamic>))
      .toList(),
  total: (json['total'] as num).toInt(),
);

Map<String, dynamic> _$TicketPageToJson(TicketPage instance) =>
    <String, dynamic>{'items': instance.items, 'total': instance.total};
