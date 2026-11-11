// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'route_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

RouteInput _$RouteInputFromJson(Map<String, dynamic> json) => RouteInput(
  departmentId: json['department_id'] as String,
  assignedUserId: json['assigned_user_id'] as String?,
  note: json['note'] as String?,
);

Map<String, dynamic> _$RouteInputToJson(RouteInput instance) =>
    <String, dynamic>{
      'assigned_user_id': ?instance.assignedUserId,
      'department_id': instance.departmentId,
      'note': ?instance.note,
    };
