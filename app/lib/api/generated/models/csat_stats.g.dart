// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'csat_stats.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

CsatStats _$CsatStatsFromJson(Map<String, dynamic> json) => CsatStats(
  bad: (json['bad'] as num).toInt(),
  good: (json['good'] as num).toInt(),
  responses: (json['responses'] as num).toInt(),
  averagePercent: (json['average_percent'] as num?)?.toDouble(),
);

Map<String, dynamic> _$CsatStatsToJson(CsatStats instance) => <String, dynamic>{
  'average_percent': ?instance.averagePercent,
  'bad': instance.bad,
  'good': instance.good,
  'responses': instance.responses,
};
