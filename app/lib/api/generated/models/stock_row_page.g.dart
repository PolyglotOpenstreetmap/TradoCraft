// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'stock_row_page.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

StockRowPage _$StockRowPageFromJson(Map<String, dynamic> json) => StockRowPage(
  items: (json['items'] as List<dynamic>)
      .map((e) => StockRow.fromJson(e as Map<String, dynamic>))
      .toList(),
  total: (json['total'] as num).toInt(),
);

Map<String, dynamic> _$StockRowPageToJson(StockRowPage instance) =>
    <String, dynamic>{'items': instance.items, 'total': instance.total};
