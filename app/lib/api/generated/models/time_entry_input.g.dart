// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'time_entry_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

TimeEntryInput _$TimeEntryInputFromJson(Map<String, dynamic> json) =>
    TimeEntryInput(
      minutes: (json['minutes'] as num).toInt(),
      billable: json['billable'] as bool?,
      note: json['note'] as String?,
    );

Map<String, dynamic> _$TimeEntryInputToJson(TimeEntryInput instance) =>
    <String, dynamic>{
      'billable': ?instance.billable,
      'minutes': instance.minutes,
      'note': ?instance.note,
    };
