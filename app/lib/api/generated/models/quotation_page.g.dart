// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'quotation_page.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

QuotationPage _$QuotationPageFromJson(Map<String, dynamic> json) =>
    QuotationPage(
      items: (json['items'] as List<dynamic>)
          .map((e) => QuotationSummary.fromJson(e as Map<String, dynamic>))
          .toList(),
      total: (json['total'] as num).toInt(),
    );

Map<String, dynamic> _$QuotationPageToJson(QuotationPage instance) =>
    <String, dynamic>{'items': instance.items, 'total': instance.total};
