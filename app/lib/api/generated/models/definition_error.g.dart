// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'definition_error.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

DefinitionError _$DefinitionErrorFromJson(Map<String, dynamic> json) =>
    DefinitionError(
      message: json['message'] as String,
      path: json['path'] as String,
      end: (json['end'] as num?)?.toInt(),
      start: (json['start'] as num?)?.toInt(),
    );

Map<String, dynamic> _$DefinitionErrorToJson(DefinitionError instance) =>
    <String, dynamic>{
      'end': ?instance.end,
      'message': instance.message,
      'path': instance.path,
      'start': ?instance.start,
    };
