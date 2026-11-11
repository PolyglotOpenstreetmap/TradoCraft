// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'priority_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

PriorityInput _$PriorityInputFromJson(Map<String, dynamic> json) =>
    PriorityInput(
      priority: json['priority'] as String,
      reason: json['reason'] as String,
      requestId: json['request_id'] as String?,
    );

Map<String, dynamic> _$PriorityInputToJson(PriorityInput instance) =>
    <String, dynamic>{
      'priority': instance.priority,
      'reason': instance.reason,
      'request_id': ?instance.requestId,
    };
