// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'sla_policy.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

SlaPolicy _$SlaPolicyFromJson(Map<String, dynamic> json) => SlaPolicy(
  firstResponseMinutes: (json['first_response_minutes'] as num).toInt(),
  priority: json['priority'] as String,
  resolutionMinutes: (json['resolution_minutes'] as num).toInt(),
);

Map<String, dynamic> _$SlaPolicyToJson(SlaPolicy instance) => <String, dynamic>{
  'first_response_minutes': instance.firstResponseMinutes,
  'priority': instance.priority,
  'resolution_minutes': instance.resolutionMinutes,
};
