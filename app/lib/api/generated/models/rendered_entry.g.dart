// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'rendered_entry.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

RenderedEntry _$RenderedEntryFromJson(Map<String, dynamic> json) =>
    RenderedEntry(
      entryId: json['entry_id'] as String,
      fallback: json['fallback'] as bool,
      language: json['language'] as String,
      summary: json['summary'] as String,
      text: json['text'] as String,
      title: json['title'] as String,
      link: json['link'] as String?,
    );

Map<String, dynamic> _$RenderedEntryToJson(RenderedEntry instance) =>
    <String, dynamic>{
      'entry_id': instance.entryId,
      'fallback': instance.fallback,
      'language': instance.language,
      'link': ?instance.link,
      'summary': instance.summary,
      'text': instance.text,
      'title': instance.title,
    };
