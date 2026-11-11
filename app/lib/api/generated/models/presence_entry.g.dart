// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'presence_entry.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

PresenceEntry _$PresenceEntryFromJson(Map<String, dynamic> json) =>
    PresenceEntry(
      mode: json['mode'] as String,
      seenAt: DateTime.parse(json['seen_at'] as String),
      userId: json['user_id'] as String,
      userName: json['user_name'] as String,
    );

Map<String, dynamic> _$PresenceEntryToJson(PresenceEntry instance) =>
    <String, dynamic>{
      'mode': instance.mode,
      'seen_at': instance.seenAt.toIso8601String(),
      'user_id': instance.userId,
      'user_name': instance.userName,
    };
