// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'repair_page.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

RepairPage _$RepairPageFromJson(Map<String, dynamic> json) => RepairPage(
  items: (json['items'] as List<dynamic>)
      .map((e) => Repair.fromJson(e as Map<String, dynamic>))
      .toList(),
  total: (json['total'] as num).toInt(),
);

Map<String, dynamic> _$RepairPageToJson(RepairPage instance) =>
    <String, dynamic>{'items': instance.items, 'total': instance.total};
