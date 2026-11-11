// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'count_entries.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

CountEntries _$CountEntriesFromJson(Map<String, dynamic> json) => CountEntries(
  entries: (json['entries'] as List<dynamic>)
      .map((e) => CountEntry.fromJson(e as Map<String, dynamic>))
      .toList(),
);

Map<String, dynamic> _$CountEntriesToJson(CountEntries instance) =>
    <String, dynamic>{'entries': instance.entries};
