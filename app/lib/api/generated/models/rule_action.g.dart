// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'rule_action.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

RuleAction _$RuleActionFromJson(Map<String, dynamic> json) => RuleAction(
  kind: json['kind'] as String,
  flag: json['flag'] as bool?,
  teamId: json['team_id'] as String?,
  userId: json['user_id'] as String?,
  value: json['value'] as String?,
);

Map<String, dynamic> _$RuleActionToJson(RuleAction instance) =>
    <String, dynamic>{
      'flag': ?instance.flag,
      'kind': instance.kind,
      'team_id': ?instance.teamId,
      'user_id': ?instance.userId,
      'value': ?instance.value,
    };
