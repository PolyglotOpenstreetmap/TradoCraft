// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'product_page.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ProductPage _$ProductPageFromJson(Map<String, dynamic> json) => ProductPage(
  items: (json['items'] as List<dynamic>)
      .map((e) => Product.fromJson(e as Map<String, dynamic>))
      .toList(),
  total: (json['total'] as num).toInt(),
);

Map<String, dynamic> _$ProductPageToJson(ProductPage instance) =>
    <String, dynamic>{'items': instance.items, 'total': instance.total};
