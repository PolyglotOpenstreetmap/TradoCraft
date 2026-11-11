// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'reader.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Reader _$ReaderFromJson(Map<String, dynamic> json) => Reader(
  lastReadAt: DateTime.parse(json['last_read_at'] as String),
  userId: json['user_id'] as String,
  userName: json['user_name'] as String,
);

Map<String, dynamic> _$ReaderToJson(Reader instance) => <String, dynamic>{
  'last_read_at': instance.lastReadAt.toIso8601String(),
  'user_id': instance.userId,
  'user_name': instance.userName,
};
