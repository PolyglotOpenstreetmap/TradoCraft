// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'status_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

StatusInput _$StatusInputFromJson(Map<String, dynamic> json) => StatusInput(
  status: json['status'] as String,
  note: json['note'] as String?,
);

Map<String, dynamic> _$StatusInputToJson(StatusInput instance) =>
    <String, dynamic>{'note': ?instance.note, 'status': instance.status};
