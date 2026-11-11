// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'tag_count.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

TagCount _$TagCountFromJson(Map<String, dynamic> json) =>
    TagCount(count: (json['count'] as num).toInt(), tag: json['tag'] as String);

Map<String, dynamic> _$TagCountToJson(TagCount instance) => <String, dynamic>{
  'count': instance.count,
  'tag': instance.tag,
};
