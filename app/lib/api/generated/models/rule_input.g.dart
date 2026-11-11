// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'rule_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

RuleInput _$RuleInputFromJson(Map<String, dynamic> json) => RuleInput(
  actions: (json['actions'] as List<dynamic>)
      .map((e) => RuleAction.fromJson(e as Map<String, dynamic>))
      .toList(),
  name: json['name'] as String,
  trigger: json['trigger'] as String,
  active: json['active'] as bool?,
  conditions: json['conditions'] == null
      ? null
      : RuleConditions.fromJson(json['conditions'] as Map<String, dynamic>),
  stopOnMatch: json['stop_on_match'] as bool?,
);

Map<String, dynamic> _$RuleInputToJson(RuleInput instance) => <String, dynamic>{
  'actions': instance.actions,
  'active': ?instance.active,
  'conditions': ?instance.conditions,
  'name': instance.name,
  'stop_on_match': ?instance.stopOnMatch,
  'trigger': instance.trigger,
};
