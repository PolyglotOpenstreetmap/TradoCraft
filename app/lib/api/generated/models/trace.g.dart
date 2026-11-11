// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'trace.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Trace _$TraceFromJson(Map<String, dynamic> json) => Trace(
  rule: json['rule'] as String,
  rulePath: json['rule_path'] as String,
  sources: (json['sources'] as List<dynamic>)
      .map((e) => TraceSource.fromJson(e as Map<String, dynamic>))
      .toList(),
);

Map<String, dynamic> _$TraceToJson(Trace instance) => <String, dynamic>{
  'rule': instance.rule,
  'rule_path': instance.rulePath,
  'sources': instance.sources,
};
