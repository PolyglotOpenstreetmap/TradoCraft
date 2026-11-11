// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'new_priority_request.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

NewPriorityRequest _$NewPriorityRequestFromJson(Map<String, dynamic> json) =>
    NewPriorityRequest(
      message: json['message'] as String,
      priority: json['priority'] as String,
    );

Map<String, dynamic> _$NewPriorityRequestToJson(NewPriorityRequest instance) =>
    <String, dynamic>{
      'message': instance.message,
      'priority': instance.priority,
    };
