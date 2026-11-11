// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'rule_outcome.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

RuleOutcome _$RuleOutcomeFromJson(Map<String, dynamic> json) => RuleOutcome(
  changes: (json['changes'] as List<dynamic>)
      .map((e) => RuleChange.fromJson(e as Map<String, dynamic>))
      .toList(),
  result: json['result'] as String,
  ruleId: json['rule_id'] as String,
  ruleName: json['rule_name'] as String,
);

Map<String, dynamic> _$RuleOutcomeToJson(RuleOutcome instance) =>
    <String, dynamic>{
      'changes': instance.changes,
      'result': instance.result,
      'rule_id': instance.ruleId,
      'rule_name': instance.ruleName,
    };
