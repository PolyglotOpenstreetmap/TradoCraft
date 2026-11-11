// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'time_row.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

TimeRow _$TimeRowFromJson(Map<String, dynamic> json) => TimeRow(
  billableMinutes: (json['billable_minutes'] as num).toInt(),
  name: json['name'] as String,
  nonBillableMinutes: (json['non_billable_minutes'] as num).toInt(),
  totalMinutes: (json['total_minutes'] as num).toInt(),
  userId: json['user_id'] as String,
);

Map<String, dynamic> _$TimeRowToJson(TimeRow instance) => <String, dynamic>{
  'billable_minutes': instance.billableMinutes,
  'name': instance.name,
  'non_billable_minutes': instance.nonBillableMinutes,
  'total_minutes': instance.totalMinutes,
  'user_id': instance.userId,
};
