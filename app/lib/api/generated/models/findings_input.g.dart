// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'findings_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

FindingsInput _$FindingsInputFromJson(Map<String, dynamic> json) =>
    FindingsInput(
      findings: json['findings'] as String,
      diagnosis: json['diagnosis'] as String?,
    );

Map<String, dynamic> _$FindingsInputToJson(FindingsInput instance) =>
    <String, dynamic>{
      'diagnosis': ?instance.diagnosis,
      'findings': instance.findings,
    };
