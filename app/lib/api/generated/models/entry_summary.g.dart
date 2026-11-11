// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'entry_summary.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

EntrySummary _$EntrySummaryFromJson(Map<String, dynamic> json) => EntrySummary(
  fallback: json['fallback'] as bool,
  helpfulNo: (json['helpful_no'] as num).toInt(),
  helpfulYes: (json['helpful_yes'] as num).toInt(),
  id: json['id'] as String,
  language: json['language'] as String,
  number: json['number'] as String,
  slug: json['slug'] as String,
  snippet: json['snippet'] as String,
  status: json['status'] as String,
  tags: (json['tags'] as List<dynamic>).map((e) => e as String).toList(),
  title: json['title'] as String,
  updatedAt: DateTime.parse(json['updated_at'] as String),
  viewCount: (json['view_count'] as num).toInt(),
  visibility: json['visibility'] as String,
  categoryId: json['category_id'] as String?,
  publishedAt: json['published_at'] == null
      ? null
      : DateTime.parse(json['published_at'] as String),
  score: (json['score'] as num?)?.toDouble(),
);

Map<String, dynamic> _$EntrySummaryToJson(EntrySummary instance) =>
    <String, dynamic>{
      'category_id': ?instance.categoryId,
      'fallback': instance.fallback,
      'helpful_no': instance.helpfulNo,
      'helpful_yes': instance.helpfulYes,
      'id': instance.id,
      'language': instance.language,
      'number': instance.number,
      'published_at': ?instance.publishedAt?.toIso8601String(),
      'score': ?instance.score,
      'slug': instance.slug,
      'snippet': instance.snippet,
      'status': instance.status,
      'tags': instance.tags,
      'title': instance.title,
      'updated_at': instance.updatedAt.toIso8601String(),
      'view_count': instance.viewCount,
      'visibility': instance.visibility,
    };
