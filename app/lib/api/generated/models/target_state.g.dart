// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'target_state.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

TargetState _$TargetStateFromJson(Map<String, dynamic> json) => TargetState(
  atRisk: json['at_risk'] as bool,
  breached: json['breached'] as bool,
  dueAt: json['due_at'] == null
      ? null
      : DateTime.parse(json['due_at'] as String),
  metAt: json['met_at'] == null
      ? null
      : DateTime.parse(json['met_at'] as String),
  minutesRemaining: (json['minutes_remaining'] as num?)?.toInt(),
);

Map<String, dynamic> _$TargetStateToJson(TargetState instance) =>
    <String, dynamic>{
      'at_risk': instance.atRisk,
      'breached': instance.breached,
      'due_at': ?instance.dueAt?.toIso8601String(),
      'met_at': ?instance.metAt?.toIso8601String(),
      'minutes_remaining': ?instance.minutesRemaining,
    };
