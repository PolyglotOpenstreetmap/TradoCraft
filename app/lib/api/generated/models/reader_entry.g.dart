// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'reader_entry.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ReaderEntry _$ReaderEntryFromJson(Map<String, dynamic> json) => ReaderEntry(
  availableLanguages: (json['available_languages'] as List<dynamic>)
      .map((e) => e as String)
      .toList(),
  body: json['body'] as String,
  fallback: json['fallback'] as bool,
  id: json['id'] as String,
  images: (json['images'] as List<dynamic>)
      .map((e) => ReaderImage.fromJson(e as Map<String, dynamic>))
      .toList(),
  language: json['language'] as String,
  number: json['number'] as String,
  related: (json['related'] as List<dynamic>)
      .map((e) => ReaderLink.fromJson(e as Map<String, dynamic>))
      .toList(),
  slug: json['slug'] as String,
  summary: json['summary'] as String,
  tags: (json['tags'] as List<dynamic>).map((e) => e as String).toList(),
  title: json['title'] as String,
  updatedAt: DateTime.parse(json['updated_at'] as String),
  categoryId: json['category_id'] as String?,
  myVote: json['my_vote'] as bool?,
);

Map<String, dynamic> _$ReaderEntryToJson(ReaderEntry instance) =>
    <String, dynamic>{
      'available_languages': instance.availableLanguages,
      'body': instance.body,
      'category_id': ?instance.categoryId,
      'fallback': instance.fallback,
      'id': instance.id,
      'images': instance.images,
      'language': instance.language,
      'my_vote': ?instance.myVote,
      'number': instance.number,
      'related': instance.related,
      'slug': instance.slug,
      'summary': instance.summary,
      'tags': instance.tags,
      'title': instance.title,
      'updated_at': instance.updatedAt.toIso8601String(),
    };
