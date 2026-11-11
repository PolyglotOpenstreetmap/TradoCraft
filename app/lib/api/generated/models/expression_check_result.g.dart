// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'expression_check_result.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ExpressionCheckResult _$ExpressionCheckResultFromJson(
  Map<String, dynamic> json,
) => ExpressionCheckResult(
  errors: (json['errors'] as List<dynamic>)
      .map((e) => ExprError.fromJson(e as Map<String, dynamic>))
      .toList(),
  ok: json['ok'] as bool,
  type: json['type'] as String,
);

Map<String, dynamic> _$ExpressionCheckResultToJson(
  ExpressionCheckResult instance,
) => <String, dynamic>{
  'errors': instance.errors,
  'ok': instance.ok,
  'type': instance.type,
};
