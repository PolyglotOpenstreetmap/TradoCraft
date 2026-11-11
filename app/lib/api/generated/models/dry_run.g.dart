// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'dry_run.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

DryRun _$DryRunFromJson(Map<String, dynamic> json) => DryRun(
  rules: (json['rules'] as List<dynamic>)
      .map((e) => RuleOutcome.fromJson(e as Map<String, dynamic>))
      .toList(),
  ticketNumber: json['ticket_number'] as String,
  trigger: json['trigger'] as String,
);

Map<String, dynamic> _$DryRunToJson(DryRun instance) => <String, dynamic>{
  'rules': instance.rules,
  'ticket_number': instance.ticketNumber,
  'trigger': instance.trigger,
};
