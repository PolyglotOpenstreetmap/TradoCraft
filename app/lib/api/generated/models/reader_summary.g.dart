// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'reader_summary.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ReaderSummary _$ReaderSummaryFromJson(Map<String, dynamic> json) =>
    ReaderSummary(
      fallback: json['fallback'] as bool,
      id: json['id'] as String,
      language: json['language'] as String,
      number: json['number'] as String,
      slug: json['slug'] as String,
      snippet: json['snippet'] as String,
      tags: (json['tags'] as List<dynamic>).map((e) => e as String).toList(),
      title: json['title'] as String,
      updatedAt: DateTime.parse(json['updated_at'] as String),
      categoryId: json['category_id'] as String?,
    );

Map<String, dynamic> _$ReaderSummaryToJson(ReaderSummary instance) =>
    <String, dynamic>{
      'category_id': ?instance.categoryId,
      'fallback': instance.fallback,
      'id': instance.id,
      'language': instance.language,
      'number': instance.number,
      'slug': instance.slug,
      'snippet': instance.snippet,
      'tags': instance.tags,
      'title': instance.title,
      'updated_at': instance.updatedAt.toIso8601String(),
    };
