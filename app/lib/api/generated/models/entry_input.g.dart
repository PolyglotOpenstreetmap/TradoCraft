// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'entry_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

EntryInput _$EntryInputFromJson(Map<String, dynamic> json) => EntryInput(
  texts: (json['texts'] as List<dynamic>)
      .map((e) => EntryText.fromJson(e as Map<String, dynamic>))
      .toList(),
  visibility: json['visibility'] as String,
  categoryId: json['category_id'] as String?,
  productIds: (json['product_ids'] as List<dynamic>?)
      ?.map((e) => e as String)
      .toList(),
  relatedIds: (json['related_ids'] as List<dynamic>?)
      ?.map((e) => e as String)
      .toList(),
  tags: (json['tags'] as List<dynamic>?)?.map((e) => e as String).toList(),
  ticketCategories: (json['ticket_categories'] as List<dynamic>?)
      ?.map((e) => e as String)
      .toList(),
);

Map<String, dynamic> _$EntryInputToJson(EntryInput instance) =>
    <String, dynamic>{
      'category_id': ?instance.categoryId,
      'product_ids': ?instance.productIds,
      'related_ids': ?instance.relatedIds,
      'tags': ?instance.tags,
      'texts': instance.texts,
      'ticket_categories': ?instance.ticketCategories,
      'visibility': instance.visibility,
    };
