// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'product_stock_page.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ProductStockPage _$ProductStockPageFromJson(Map<String, dynamic> json) =>
    ProductStockPage(
      items: (json['items'] as List<dynamic>)
          .map((e) => ProductStock.fromJson(e as Map<String, dynamic>))
          .toList(),
      total: (json['total'] as num).toInt(),
    );

Map<String, dynamic> _$ProductStockPageToJson(ProductStockPage instance) =>
    <String, dynamic>{'items': instance.items, 'total': instance.total};
