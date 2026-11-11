// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'task_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

TaskInput _$TaskInputFromJson(Map<String, dynamic> json) => TaskInput(
  description: json['description'] as String,
  clientId: json['client_id'] as String?,
);

Map<String, dynamic> _$TaskInputToJson(TaskInput instance) => <String, dynamic>{
  'client_id': ?instance.clientId,
  'description': instance.description,
};
