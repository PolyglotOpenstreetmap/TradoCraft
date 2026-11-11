// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'revision_update.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

RevisionUpdate _$RevisionUpdateFromJson(Map<String, dynamic> json) =>
    RevisionUpdate(
      definition: json['definition'],
      notes: json['notes'] as String?,
    );

Map<String, dynamic> _$RevisionUpdateToJson(RevisionUpdate instance) =>
    <String, dynamic>{
      'definition': instance.definition,
      'notes': ?instance.notes,
    };
