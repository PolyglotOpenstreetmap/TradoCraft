// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'package_page.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

PackagePage _$PackagePageFromJson(Map<String, dynamic> json) => PackagePage(
  items: (json['items'] as List<dynamic>)
      .map((e) => Package.fromJson(e as Map<String, dynamic>))
      .toList(),
  total: (json['total'] as num).toInt(),
);

Map<String, dynamic> _$PackagePageToJson(PackagePage instance) =>
    <String, dynamic>{'items': instance.items, 'total': instance.total};
