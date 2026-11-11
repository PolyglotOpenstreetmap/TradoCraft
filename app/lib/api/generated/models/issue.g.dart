// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'issue.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Issue _$IssueFromJson(Map<String, dynamic> json) => Issue(
  code: json['code'] as String,
  message: json['message'] as String,
  scope: json['scope'] as String,
  severity: IssueSeverity.fromJson(json['severity'] as String),
  field: json['field'] as String?,
  rule: json['rule'] as String?,
);

Map<String, dynamic> _$IssueToJson(Issue instance) => <String, dynamic>{
  'code': instance.code,
  'field': ?instance.field,
  'message': instance.message,
  'rule': ?instance.rule,
  'scope': instance.scope,
  'severity': instance.severity,
};
