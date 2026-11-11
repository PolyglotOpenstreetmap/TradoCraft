// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'rendered_answer.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

RenderedAnswer _$RenderedAnswerFromJson(Map<String, dynamic> json) =>
    RenderedAnswer(
      answerId: json['answer_id'] as String,
      body: json['body'] as String,
      fallback: json['fallback'] as bool,
      language: json['language'] as String,
      subject: json['subject'] as String,
    );

Map<String, dynamic> _$RenderedAnswerToJson(RenderedAnswer instance) =>
    <String, dynamic>{
      'answer_id': instance.answerId,
      'body': instance.body,
      'fallback': instance.fallback,
      'language': instance.language,
      'subject': instance.subject,
    };
