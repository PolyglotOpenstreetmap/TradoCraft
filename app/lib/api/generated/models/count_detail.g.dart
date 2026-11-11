// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'count_detail.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

CountDetail _$CountDetailFromJson(Map<String, dynamic> json) => CountDetail(
  count: Count.fromJson(json['count'] as Map<String, dynamic>),
  lines: (json['lines'] as List<dynamic>)
      .map((e) => CountLine.fromJson(e as Map<String, dynamic>))
      .toList(),
  movedDuringCount: (json['moved_during_count'] as num).toInt(),
);

Map<String, dynamic> _$CountDetailToJson(CountDetail instance) =>
    <String, dynamic>{
      'count': instance.count,
      'lines': instance.lines,
      'moved_during_count': instance.movedDuringCount,
    };
