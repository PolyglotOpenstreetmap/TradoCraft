// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'time_entries.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

TimeEntries _$TimeEntriesFromJson(Map<String, dynamic> json) => TimeEntries(
  billableMinutes: (json['billable_minutes'] as num).toInt(),
  items: (json['items'] as List<dynamic>)
      .map((e) => TimeEntry.fromJson(e as Map<String, dynamic>))
      .toList(),
  totalMinutes: (json['total_minutes'] as num).toInt(),
);

Map<String, dynamic> _$TimeEntriesToJson(TimeEntries instance) =>
    <String, dynamic>{
      'billable_minutes': instance.billableMinutes,
      'items': instance.items,
      'total_minutes': instance.totalMinutes,
    };
