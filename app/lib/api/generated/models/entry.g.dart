// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'entry.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Entry _$EntryFromJson(Map<String, dynamic> json) => Entry(
  createdAt: DateTime.parse(json['created_at'] as String),
  helpfulNo: (json['helpful_no'] as num).toInt(),
  helpfulYes: (json['helpful_yes'] as num).toInt(),
  id: json['id'] as String,
  images: (json['images'] as List<dynamic>)
      .map((e) => FileInfo.fromJson(e as Map<String, dynamic>))
      .toList(),
  number: json['number'] as String,
  productIds: (json['product_ids'] as List<dynamic>)
      .map((e) => e as String)
      .toList(),
  related: (json['related'] as List<dynamic>)
      .map((e) => RelatedEntry.fromJson(e as Map<String, dynamic>))
      .toList(),
  slug: json['slug'] as String,
  status: json['status'] as String,
  tags: (json['tags'] as List<dynamic>).map((e) => e as String).toList(),
  texts: (json['texts'] as List<dynamic>)
      .map((e) => EntryText.fromJson(e as Map<String, dynamic>))
      .toList(),
  ticketCategories: (json['ticket_categories'] as List<dynamic>)
      .map((e) => e as String)
      .toList(),
  updatedAt: DateTime.parse(json['updated_at'] as String),
  version: (json['version'] as num).toInt(),
  viewCount: (json['view_count'] as num).toInt(),
  visibility: json['visibility'] as String,
  authorId: json['author_id'] as String?,
  authorName: json['author_name'] as String?,
  categoryId: json['category_id'] as String?,
  publishedAt: json['published_at'] == null
      ? null
      : DateTime.parse(json['published_at'] as String),
);

Map<String, dynamic> _$EntryToJson(Entry instance) => <String, dynamic>{
  'author_id': ?instance.authorId,
  'author_name': ?instance.authorName,
  'category_id': ?instance.categoryId,
  'created_at': instance.createdAt.toIso8601String(),
  'helpful_no': instance.helpfulNo,
  'helpful_yes': instance.helpfulYes,
  'id': instance.id,
  'images': instance.images,
  'number': instance.number,
  'product_ids': instance.productIds,
  'published_at': ?instance.publishedAt?.toIso8601String(),
  'related': instance.related,
  'slug': instance.slug,
  'status': instance.status,
  'tags': instance.tags,
  'texts': instance.texts,
  'ticket_categories': instance.ticketCategories,
  'updated_at': instance.updatedAt.toIso8601String(),
  'version': instance.version,
  'view_count': instance.viewCount,
  'visibility': instance.visibility,
};
