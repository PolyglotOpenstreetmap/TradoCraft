// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'production_page.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ProductionPage _$ProductionPageFromJson(Map<String, dynamic> json) =>
    ProductionPage(
      items: (json['items'] as List<dynamic>)
          .map((e) => ProductionSummary.fromJson(e as Map<String, dynamic>))
          .toList(),
      total: (json['total'] as num).toInt(),
    );

Map<String, dynamic> _$ProductionPageToJson(ProductionPage instance) =>
    <String, dynamic>{'items': instance.items, 'total': instance.total};
