// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'count_page.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

CountPage _$CountPageFromJson(Map<String, dynamic> json) => CountPage(
  items: (json['items'] as List<dynamic>)
      .map((e) => Count.fromJson(e as Map<String, dynamic>))
      .toList(),
  total: (json['total'] as num).toInt(),
);

Map<String, dynamic> _$CountPageToJson(CountPage instance) => <String, dynamic>{
  'items': instance.items,
  'total': instance.total,
};
