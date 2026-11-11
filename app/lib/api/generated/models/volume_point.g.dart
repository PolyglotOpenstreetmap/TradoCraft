// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'volume_point.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

VolumePoint _$VolumePointFromJson(Map<String, dynamic> json) => VolumePoint(
  closed: (json['closed'] as num).toInt(),
  created: (json['created'] as num).toInt(),
  period: DateTime.parse(json['period'] as String),
);

Map<String, dynamic> _$VolumePointToJson(VolumePoint instance) =>
    <String, dynamic>{
      'closed': instance.closed,
      'created': instance.created,
      'period': instance.period.toIso8601String(),
    };
