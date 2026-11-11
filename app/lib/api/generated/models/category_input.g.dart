// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'category_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

CategoryInput _$CategoryInputFromJson(Map<String, dynamic> json) =>
    CategoryInput(
      names: (json['names'] as List<dynamic>)
          .map((e) => CategoryName.fromJson(e as Map<String, dynamic>))
          .toList(),
      active: json['active'] as bool?,
      parentId: json['parent_id'] as String?,
      sortOrder: (json['sort_order'] as num?)?.toInt(),
    );

Map<String, dynamic> _$CategoryInputToJson(CategoryInput instance) =>
    <String, dynamic>{
      'active': ?instance.active,
      'names': instance.names,
      'parent_id': ?instance.parentId,
      'sort_order': ?instance.sortOrder,
    };
