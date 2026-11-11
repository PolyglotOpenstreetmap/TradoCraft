// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'expression_check_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ExpressionCheckInput _$ExpressionCheckInputFromJson(
  Map<String, dynamic> json,
) => ExpressionCheckInput(
  source: json['source'] as String,
  definition: json['definition'],
  locals: (json['locals'] as List<dynamic>?)?.map((e) => e as String).toList(),
  scope: json['scope'] == null
      ? null
      : RuleScope.fromJson(json['scope'] as String),
  table: json['table'] as String?,
  template: json['template'] as bool?,
);

Map<String, dynamic> _$ExpressionCheckInputToJson(
  ExpressionCheckInput instance,
) => <String, dynamic>{
  'definition': ?instance.definition,
  'locals': ?instance.locals,
  'scope': ?instance.scope,
  'source': instance.source,
  'table': ?instance.table,
  'template': ?instance.template,
};
