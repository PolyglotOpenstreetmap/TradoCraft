// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'portal_message.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

PortalMessage _$PortalMessageFromJson(Map<String, dynamic> json) =>
    PortalMessage(
      attachments: (json['attachments'] as List<dynamic>)
          .map((e) => FileInfo.fromJson(e as Map<String, dynamic>))
          .toList(),
      authorName: json['author_name'] as String,
      body: json['body'] as String,
      createdAt: DateTime.parse(json['created_at'] as String),
      direction: json['direction'] as String,
      id: json['id'] as String,
    );

Map<String, dynamic> _$PortalMessageToJson(PortalMessage instance) =>
    <String, dynamic>{
      'attachments': instance.attachments,
      'author_name': instance.authorName,
      'body': instance.body,
      'created_at': instance.createdAt.toIso8601String(),
      'direction': instance.direction,
      'id': instance.id,
    };
