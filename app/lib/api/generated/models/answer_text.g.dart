// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'answer_text.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

AnswerText _$AnswerTextFromJson(Map<String, dynamic> json) => AnswerText(
  body: json['body'] as String,
  language: json['language'] as String,
  subject: json['subject'] as String?,
);

Map<String, dynamic> _$AnswerTextToJson(AnswerText instance) =>
    <String, dynamic>{
      'body': instance.body,
      'language': instance.language,
      'subject': ?instance.subject,
    };
