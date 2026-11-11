// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'task.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Task _$TaskFromJson(Map<String, dynamic> json) => Task(
  description: json['description'] as String,
  done: json['done'] as bool,
  id: json['id'] as String,
  position: (json['position'] as num).toInt(),
  doneAt: json['done_at'] == null
      ? null
      : DateTime.parse(json['done_at'] as String),
  doneByName: json['done_by_name'] as String?,
);

Map<String, dynamic> _$TaskToJson(Task instance) => <String, dynamic>{
  'description': instance.description,
  'done': instance.done,
  'done_at': ?instance.doneAt?.toIso8601String(),
  'done_by_name': ?instance.doneByName,
  'id': instance.id,
  'position': instance.position,
};
