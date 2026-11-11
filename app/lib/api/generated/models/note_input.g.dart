// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'note_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

NoteInput _$NoteInputFromJson(Map<String, dynamic> json) => NoteInput(
  body: json['body'] as String,
  attachmentIds: (json['attachment_ids'] as List<dynamic>?)
      ?.map((e) => e as String)
      .toList(),
  status: json['status'] as String?,
);

Map<String, dynamic> _$NoteInputToJson(NoteInput instance) => <String, dynamic>{
  'attachment_ids': ?instance.attachmentIds,
  'body': instance.body,
  'status': ?instance.status,
};
