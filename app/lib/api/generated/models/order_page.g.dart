// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'order_page.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

OrderPage _$OrderPageFromJson(Map<String, dynamic> json) => OrderPage(
  items: (json['items'] as List<dynamic>)
      .map((e) => OrderSummary.fromJson(e as Map<String, dynamic>))
      .toList(),
  total: (json['total'] as num).toInt(),
);

Map<String, dynamic> _$OrderPageToJson(OrderPage instance) => <String, dynamic>{
  'items': instance.items,
  'total': instance.total,
};
