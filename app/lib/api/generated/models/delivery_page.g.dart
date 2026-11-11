// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'delivery_page.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

DeliveryPage _$DeliveryPageFromJson(Map<String, dynamic> json) => DeliveryPage(
  items: (json['items'] as List<dynamic>)
      .map((e) => DeliverySummary.fromJson(e as Map<String, dynamic>))
      .toList(),
  total: (json['total'] as num).toInt(),
);

Map<String, dynamic> _$DeliveryPageToJson(DeliveryPage instance) =>
    <String, dynamic>{'items': instance.items, 'total': instance.total};
