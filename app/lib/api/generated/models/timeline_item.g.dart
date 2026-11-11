// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'timeline_item.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

TimelineItem _$TimelineItemFromJson(Map<String, dynamic> json) => TimelineItem(
  at: DateTime.parse(json['at'] as String),
  comment: json['comment'] as String,
  id: json['id'] as String,
  kind: json['kind'] as String,
  source: json['source'] as String,
  actorName: json['actor_name'] as String?,
  actorUserId: json['actor_user_id'] as String?,
  fromValue: json['from_value'] as String?,
  previousUserId: json['previous_user_id'] as String?,
  previousUserName: json['previous_user_name'] as String?,
  targetTeamId: json['target_team_id'] as String?,
  targetTeamName: json['target_team_name'] as String?,
  targetUserId: json['target_user_id'] as String?,
  targetUserName: json['target_user_name'] as String?,
  toValue: json['to_value'] as String?,
);

Map<String, dynamic> _$TimelineItemToJson(TimelineItem instance) =>
    <String, dynamic>{
      'actor_name': ?instance.actorName,
      'actor_user_id': ?instance.actorUserId,
      'at': instance.at.toIso8601String(),
      'comment': instance.comment,
      'from_value': ?instance.fromValue,
      'id': instance.id,
      'kind': instance.kind,
      'previous_user_id': ?instance.previousUserId,
      'previous_user_name': ?instance.previousUserName,
      'source': instance.source,
      'target_team_id': ?instance.targetTeamId,
      'target_team_name': ?instance.targetTeamName,
      'target_user_id': ?instance.targetUserId,
      'target_user_name': ?instance.targetUserName,
      'to_value': ?instance.toValue,
    };
