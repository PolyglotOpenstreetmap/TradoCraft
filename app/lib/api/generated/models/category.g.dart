// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'category.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Category _$CategoryFromJson(Map<String, dynamic> json) => Category(
  active: json['active'] as bool,
  entryCount: (json['entry_count'] as num).toInt(),
  id: json['id'] as String,
  names: (json['names'] as List<dynamic>)
      .map((e) => CategoryName.fromJson(e as Map<String, dynamic>))
      .toList(),
  sortOrder: (json['sort_order'] as num).toInt(),
  parentId: json['parent_id'] as String?,
);

Map<String, dynamic> _$CategoryToJson(Category instance) => <String, dynamic>{
  'active': instance.active,
  'entry_count': instance.entryCount,
  'id': instance.id,
  'names': instance.names,
  'parent_id': ?instance.parentId,
  'sort_order': instance.sortOrder,
};
