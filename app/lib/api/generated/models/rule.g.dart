// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'rule.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Rule _$RuleFromJson(Map<String, dynamic> json) => Rule(
  actions: (json['actions'] as List<dynamic>)
      .map((e) => RuleAction.fromJson(e as Map<String, dynamic>))
      .toList(),
  active: json['active'] as bool,
  conditions: RuleConditions.fromJson(
    json['conditions'] as Map<String, dynamic>,
  ),
  id: json['id'] as String,
  name: json['name'] as String,
  position: (json['position'] as num).toInt(),
  stopOnMatch: json['stop_on_match'] as bool,
  trigger: json['trigger'] as String,
  updatedAt: DateTime.parse(json['updated_at'] as String),
);

Map<String, dynamic> _$RuleToJson(Rule instance) => <String, dynamic>{
  'actions': instance.actions,
  'active': instance.active,
  'conditions': instance.conditions,
  'id': instance.id,
  'name': instance.name,
  'position': instance.position,
  'stop_on_match': instance.stopOnMatch,
  'trigger': instance.trigger,
  'updated_at': instance.updatedAt.toIso8601String(),
};
