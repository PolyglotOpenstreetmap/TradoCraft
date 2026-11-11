// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'rule_change.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

RuleChange _$RuleChangeFromJson(Map<String, dynamic> json) => RuleChange(
  kind: json['kind'] as String,
  from: json['from'] as String?,
  to: json['to'] as String?,
);

Map<String, dynamic> _$RuleChangeToJson(RuleChange instance) =>
    <String, dynamic>{
      'from': ?instance.from,
      'kind': instance.kind,
      'to': ?instance.to,
    };
