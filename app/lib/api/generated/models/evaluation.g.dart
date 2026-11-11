// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'evaluation.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Evaluation _$EvaluationFromJson(Map<String, dynamic> json) => Evaluation(
  groups: (json['groups'] as List<dynamic>)
      .map((e) => GroupResult.fromJson(e as Map<String, dynamic>))
      .toList(),
  issues: (json['issues'] as List<dynamic>)
      .map((e) => Issue.fromJson(e as Map<String, dynamic>))
      .toList(),
  items: (json['items'] as List<dynamic>)
      .map((e) => ItemTableResult.fromJson(e as Map<String, dynamic>))
      .toList(),
  lines: (json['lines'] as List<dynamic>)
      .map((e) => Line.fromJson(e as Map<String, dynamic>))
      .toList(),
  project: (json['project'] as List<dynamic>)
      .map((e) => ValueResult.fromJson(e as Map<String, dynamic>))
      .toList(),
  steps: (json['steps'] as num).toInt(),
);

Map<String, dynamic> _$EvaluationToJson(Evaluation instance) =>
    <String, dynamic>{
      'groups': instance.groups,
      'issues': instance.issues,
      'items': instance.items,
      'lines': instance.lines,
      'project': instance.project,
      'steps': instance.steps,
    };
