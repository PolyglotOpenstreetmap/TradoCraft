// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'repair_detail.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

RepairDetail _$RepairDetailFromJson(Map<String, dynamic> json) => RepairDetail(
  events: (json['events'] as List<dynamic>)
      .map((e) => RepairEvent.fromJson(e as Map<String, dynamic>))
      .toList(),
  photos: (json['photos'] as List<dynamic>)
      .map((e) => Photo.fromJson(e as Map<String, dynamic>))
      .toList(),
  repair: Repair.fromJson(json['repair'] as Map<String, dynamic>),
);

Map<String, dynamic> _$RepairDetailToJson(RepairDetail instance) =>
    <String, dynamic>{
      'events': instance.events,
      'photos': instance.photos,
      'repair': instance.repair,
    };
