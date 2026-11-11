// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'ticket_sla.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

TicketSla _$TicketSlaFromJson(Map<String, dynamic> json) => TicketSla(
  firstResponse: TargetState.fromJson(
    json['first_response'] as Map<String, dynamic>,
  ),
  firstResponseBreached: json['first_response_breached'] as bool,
  paused: json['paused'] as bool,
  resolution: TargetState.fromJson(json['resolution'] as Map<String, dynamic>),
  resolutionBreached: json['resolution_breached'] as bool,
  state: json['state'] as String,
);

Map<String, dynamic> _$TicketSlaToJson(TicketSla instance) => <String, dynamic>{
  'first_response': instance.firstResponse,
  'first_response_breached': instance.firstResponseBreached,
  'paused': instance.paused,
  'resolution': instance.resolution,
  'resolution_breached': instance.resolutionBreached,
  'state': instance.state,
};
