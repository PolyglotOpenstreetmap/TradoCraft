// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'new_revision_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

NewRevisionInput _$NewRevisionInputFromJson(Map<String, dynamic> json) =>
    NewRevisionInput(
      copyFrom: json['copy_from'] as String?,
      notes: json['notes'] as String?,
    );

Map<String, dynamic> _$NewRevisionInputToJson(NewRevisionInput instance) =>
    <String, dynamic>{
      'copy_from': ?instance.copyFrom,
      'notes': ?instance.notes,
    };
