// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'closure_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ClosureInput _$ClosureInputFromJson(Map<String, dynamic> json) => ClosureInput(
  startsOn: DateTime.parse(json['starts_on'] as String),
  endsOn: json['ends_on'] == null
      ? null
      : DateTime.parse(json['ends_on'] as String),
  kind: json['kind'] as String?,
  notes: json['notes'] as String?,
);

Map<String, dynamic> _$ClosureInputToJson(ClosureInput instance) =>
    <String, dynamic>{
      'ends_on': ?instance.endsOn?.toIso8601String(),
      'kind': ?instance.kind,
      'notes': ?instance.notes,
      'starts_on': instance.startsOn.toIso8601String(),
    };
