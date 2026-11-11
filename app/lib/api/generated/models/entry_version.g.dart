// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'entry_version.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

EntryVersion _$EntryVersionFromJson(Map<String, dynamic> json) => EntryVersion(
  editedAt: DateTime.parse(json['edited_at'] as String),
  texts: (json['texts'] as List<dynamic>)
      .map((e) => EntryText.fromJson(e as Map<String, dynamic>))
      .toList(),
  version: (json['version'] as num).toInt(),
  editorId: json['editor_id'] as String?,
  editorName: json['editor_name'] as String?,
  restoredFrom: (json['restored_from'] as num?)?.toInt(),
);

Map<String, dynamic> _$EntryVersionToJson(EntryVersion instance) =>
    <String, dynamic>{
      'edited_at': instance.editedAt.toIso8601String(),
      'editor_id': ?instance.editorId,
      'editor_name': ?instance.editorName,
      'restored_from': ?instance.restoredFrom,
      'texts': instance.texts,
      'version': instance.version,
    };
