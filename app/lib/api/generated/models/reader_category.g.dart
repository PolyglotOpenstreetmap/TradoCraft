// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'reader_category.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ReaderCategory _$ReaderCategoryFromJson(Map<String, dynamic> json) =>
    ReaderCategory(
      entryCount: (json['entry_count'] as num).toInt(),
      id: json['id'] as String,
      name: json['name'] as String,
      sortOrder: (json['sort_order'] as num).toInt(),
      parentId: json['parent_id'] as String?,
    );

Map<String, dynamic> _$ReaderCategoryToJson(ReaderCategory instance) =>
    <String, dynamic>{
      'entry_count': instance.entryCount,
      'id': instance.id,
      'name': instance.name,
      'parent_id': ?instance.parentId,
      'sort_order': instance.sortOrder,
    };
