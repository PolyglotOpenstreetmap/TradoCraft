// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'absence.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Absence _$AbsenceFromJson(Map<String, dynamic> json) => Absence(
  fromDate: DateTime.parse(json['from_date'] as String),
  id: json['id'] as String,
  kind: json['kind'] as String,
  note: json['note'] as String,
  toDate: DateTime.parse(json['to_date'] as String),
  userId: json['user_id'] as String,
  userName: json['user_name'] as String,
);

Map<String, dynamic> _$AbsenceToJson(Absence instance) => <String, dynamic>{
  'from_date': instance.fromDate.toIso8601String(),
  'id': instance.id,
  'kind': instance.kind,
  'note': instance.note,
  'to_date': instance.toDate.toIso8601String(),
  'user_id': instance.userId,
  'user_name': instance.userName,
};
