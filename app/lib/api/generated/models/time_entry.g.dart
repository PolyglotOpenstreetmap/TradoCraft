// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'time_entry.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

TimeEntry _$TimeEntryFromJson(Map<String, dynamic> json) => TimeEntry(
  billable: json['billable'] as bool,
  createdAt: DateTime.parse(json['created_at'] as String),
  id: json['id'] as String,
  minutes: (json['minutes'] as num).toInt(),
  note: json['note'] as String,
  userId: json['user_id'] as String,
  userName: json['user_name'] as String,
);

Map<String, dynamic> _$TimeEntryToJson(TimeEntry instance) => <String, dynamic>{
  'billable': instance.billable,
  'created_at': instance.createdAt.toIso8601String(),
  'id': instance.id,
  'minutes': instance.minutes,
  'note': instance.note,
  'user_id': instance.userId,
  'user_name': instance.userName,
};
