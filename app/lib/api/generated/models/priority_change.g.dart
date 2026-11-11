// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'priority_change.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

PriorityChange _$PriorityChangeFromJson(Map<String, dynamic> json) =>
    PriorityChange(
      changedAt: DateTime.parse(json['changed_at'] as String),
      changedBy: json['changed_by'] as String,
      changedByName: json['changed_by_name'] as String,
      id: json['id'] as String,
      newPriority: json['new_priority'] as String,
      oldPriority: json['old_priority'] as String,
      reason: json['reason'] as String,
    );

Map<String, dynamic> _$PriorityChangeToJson(PriorityChange instance) =>
    <String, dynamic>{
      'changed_at': instance.changedAt.toIso8601String(),
      'changed_by': instance.changedBy,
      'changed_by_name': instance.changedByName,
      'id': instance.id,
      'new_priority': instance.newPriority,
      'old_priority': instance.oldPriority,
      'reason': instance.reason,
    };
