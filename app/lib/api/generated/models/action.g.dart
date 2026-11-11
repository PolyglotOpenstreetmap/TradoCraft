// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'action.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Action _$ActionFromJson(Map<String, dynamic> json) => Action(
  kind: json['kind'] as String,
  flag: json['flag'] as bool?,
  teamId: json['team_id'] as String?,
  userId: json['user_id'] as String?,
  value: json['value'] as String?,
);

Map<String, dynamic> _$ActionToJson(Action instance) => <String, dynamic>{
  'flag': ?instance.flag,
  'kind': instance.kind,
  'team_id': ?instance.teamId,
  'user_id': ?instance.userId,
  'value': ?instance.value,
};
