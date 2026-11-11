// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'bulk_price_preview.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

BulkPricePreview _$BulkPricePreviewFromJson(Map<String, dynamic> json) =>
    BulkPricePreview(
      applied: (json['applied'] as num).toInt(),
      conflicts: (json['conflicts'] as num).toInt(),
      lines: (json['lines'] as List<dynamic>)
          .map((e) => BulkPriceLine.fromJson(e as Map<String, dynamic>))
          .toList(),
      validFrom: DateTime.parse(json['valid_from'] as String),
      withoutPrice: (json['without_price'] as num).toInt(),
    );

Map<String, dynamic> _$BulkPricePreviewToJson(BulkPricePreview instance) =>
    <String, dynamic>{
      'applied': instance.applied,
      'conflicts': instance.conflicts,
      'lines': instance.lines,
      'valid_from': instance.validFrom.toIso8601String(),
      'without_price': instance.withoutPrice,
    };
