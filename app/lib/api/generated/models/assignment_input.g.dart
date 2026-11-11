// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'assignment_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

AssignmentInput _$AssignmentInputFromJson(Map<String, dynamic> json) =>
    AssignmentInput(
      ownerUserId: json['owner_user_id'] as String?,
      teamId: json['team_id'] as String?,
    );

Map<String, dynamic> _$AssignmentInputToJson(AssignmentInput instance) =>
    <String, dynamic>{
      'owner_user_id': ?instance.ownerUserId,
      'team_id': ?instance.teamId,
    };
