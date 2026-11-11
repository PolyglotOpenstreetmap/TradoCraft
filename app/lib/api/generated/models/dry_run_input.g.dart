// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'dry_run_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

DryRunInput _$DryRunInputFromJson(Map<String, dynamic> json) => DryRunInput(
  ticketId: json['ticket_id'] as String,
  trigger: json['trigger'] as String,
);

Map<String, dynamic> _$DryRunInputToJson(DryRunInput instance) =>
    <String, dynamic>{
      'ticket_id': instance.ticketId,
      'trigger': instance.trigger,
    };
