// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'message_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

MessageInput _$MessageInputFromJson(Map<String, dynamic> json) => MessageInput(
  body: json['body'] as String,
  attachmentIds: (json['attachment_ids'] as List<dynamic>?)
      ?.map((e) => e as String)
      .toList(),
);

Map<String, dynamic> _$MessageInputToJson(MessageInput instance) =>
    <String, dynamic>{
      'attachment_ids': ?instance.attachmentIds,
      'body': instance.body,
    };
