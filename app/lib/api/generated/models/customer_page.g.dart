// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'customer_page.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

CustomerPage _$CustomerPageFromJson(Map<String, dynamic> json) => CustomerPage(
  items: (json['items'] as List<dynamic>)
      .map((e) => Customer.fromJson(e as Map<String, dynamic>))
      .toList(),
  total: (json['total'] as num).toInt(),
);

Map<String, dynamic> _$CustomerPageToJson(CustomerPage instance) =>
    <String, dynamic>{'items': instance.items, 'total': instance.total};
