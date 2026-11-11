// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'escalate_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

EscalateInput _$EscalateInputFromJson(Map<String, dynamic> json) =>
    EscalateInput(
      reason: json['reason'] as String,
      raisePriority: json['raise_priority'] as bool?,
      toTeamId: json['to_team_id'] as String?,
      toUserId: json['to_user_id'] as String?,
    );

Map<String, dynamic> _$EscalateInputToJson(EscalateInput instance) =>
    <String, dynamic>{
      'raise_priority': ?instance.raisePriority,
      'reason': instance.reason,
      'to_team_id': ?instance.toTeamId,
      'to_user_id': ?instance.toUserId,
    };
