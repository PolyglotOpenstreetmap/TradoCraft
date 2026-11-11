// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'watcher.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Watcher _$WatcherFromJson(Map<String, dynamic> json) => Watcher(
  createdAt: DateTime.parse(json['created_at'] as String),
  userId: json['user_id'] as String,
  userName: json['user_name'] as String,
);

Map<String, dynamic> _$WatcherToJson(Watcher instance) => <String, dynamic>{
  'created_at': instance.createdAt.toIso8601String(),
  'user_id': instance.userId,
  'user_name': instance.userName,
};
