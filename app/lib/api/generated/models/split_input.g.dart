// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'split_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

SplitInput _$SplitInputFromJson(Map<String, dynamic> json) => SplitInput(
  articleIds: (json['article_ids'] as List<dynamic>)
      .map((e) => e as String)
      .toList(),
  subject: json['subject'] as String?,
);

Map<String, dynamic> _$SplitInputToJson(SplitInput instance) =>
    <String, dynamic>{
      'article_ids': instance.articleIds,
      'subject': ?instance.subject,
    };
