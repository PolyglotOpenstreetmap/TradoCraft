// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'outcome.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Outcome _$OutcomeFromJson(Map<String, dynamic> json) => Outcome(
  changes: (json['changes'] as List<dynamic>)
      .map((e) => Change.fromJson(e as Map<String, dynamic>))
      .toList(),
  result: json['result'] as String,
  ruleId: json['rule_id'] as String,
  ruleName: json['rule_name'] as String,
);

Map<String, dynamic> _$OutcomeToJson(Outcome instance) => <String, dynamic>{
  'changes': instance.changes,
  'result': instance.result,
  'rule_id': instance.ruleId,
  'rule_name': instance.ruleName,
};
