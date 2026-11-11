// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'trace_source.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

TraceSource _$TraceSourceFromJson(Map<String, dynamic> json) => TraceSource(
  scope: json['scope'] as String,
  values: json['values'],
  each: json['each'],
  label: json['label'] as String?,
);

Map<String, dynamic> _$TraceSourceToJson(TraceSource instance) =>
    <String, dynamic>{
      'each': ?instance.each,
      'label': ?instance.label,
      'scope': instance.scope,
      'values': instance.values,
    };
