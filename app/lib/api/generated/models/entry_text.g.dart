// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'entry_text.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

EntryText _$EntryTextFromJson(Map<String, dynamic> json) => EntryText(
  language: json['language'] as String,
  title: json['title'] as String,
  body: json['body'] as String?,
  summary: json['summary'] as String?,
);

Map<String, dynamic> _$EntryTextToJson(EntryText instance) => <String, dynamic>{
  'body': ?instance.body,
  'language': instance.language,
  'summary': ?instance.summary,
  'title': instance.title,
};
