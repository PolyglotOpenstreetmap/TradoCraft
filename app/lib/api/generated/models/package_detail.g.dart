// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'package_detail.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

PackageDetail _$PackageDetailFromJson(Map<String, dynamic> json) =>
    PackageDetail(
      events: (json['events'] as List<dynamic>)
          .map((e) => PackageEvent.fromJson(e as Map<String, dynamic>))
          .toList(),
      package: Package.fromJson(json['package'] as Map<String, dynamic>),
      photos: (json['photos'] as List<dynamic>)
          .map((e) => Photo.fromJson(e as Map<String, dynamic>))
          .toList(),
      repairs: (json['repairs'] as List<dynamic>)
          .map((e) => PackageRepairRef.fromJson(e as Map<String, dynamic>))
          .toList(),
    );

Map<String, dynamic> _$PackageDetailToJson(PackageDetail instance) =>
    <String, dynamic>{
      'events': instance.events,
      'package': instance.package,
      'photos': instance.photos,
      'repairs': instance.repairs,
    };
