// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'absence_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

AbsenceInput _$AbsenceInputFromJson(Map<String, dynamic> json) => AbsenceInput(
  fromDate: DateTime.parse(json['from_date'] as String),
  kind: json['kind'] as String,
  toDate: DateTime.parse(json['to_date'] as String),
  userId: json['user_id'] as String,
  note: json['note'] as String?,
);

Map<String, dynamic> _$AbsenceInputToJson(AbsenceInput instance) =>
    <String, dynamic>{
      'from_date': instance.fromDate.toIso8601String(),
      'kind': instance.kind,
      'note': ?instance.note,
      'to_date': instance.toDate.toIso8601String(),
      'user_id': instance.userId,
    };
