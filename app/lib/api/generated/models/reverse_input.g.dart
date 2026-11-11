// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'reverse_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ReverseInput _$ReverseInputFromJson(Map<String, dynamic> json) => ReverseInput(
  id: json['id'] as String,
  force: json['force'] as bool?,
  note: json['note'] as String?,
);

Map<String, dynamic> _$ReverseInputToJson(ReverseInput instance) =>
    <String, dynamic>{
      'force': ?instance.force,
      'id': instance.id,
      'note': ?instance.note,
    };
