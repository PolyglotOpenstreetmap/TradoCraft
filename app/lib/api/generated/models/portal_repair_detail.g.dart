// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'portal_repair_detail.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

PortalRepairDetail _$PortalRepairDetailFromJson(Map<String, dynamic> json) =>
    PortalRepairDetail(
      photos: (json['photos'] as List<dynamic>)
          .map((e) => PortalPhoto.fromJson(e as Map<String, dynamic>))
          .toList(),
      repair: PortalRepair.fromJson(json['repair'] as Map<String, dynamic>),
    );

Map<String, dynamic> _$PortalRepairDetailToJson(PortalRepairDetail instance) =>
    <String, dynamic>{'photos': instance.photos, 'repair': instance.repair};
