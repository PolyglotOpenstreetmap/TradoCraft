// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'duration_stats.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

DurationStats _$DurationStatsFromJson(Map<String, dynamic> json) =>
    DurationStats(
      count: (json['count'] as num).toInt(),
      averageMinutes: (json['average_minutes'] as num?)?.toDouble(),
      p90Minutes: (json['p90_minutes'] as num?)?.toDouble(),
    );

Map<String, dynamic> _$DurationStatsToJson(DurationStats instance) =>
    <String, dynamic>{
      'average_minutes': ?instance.averageMinutes,
      'count': instance.count,
      'p90_minutes': ?instance.p90Minutes,
    };
