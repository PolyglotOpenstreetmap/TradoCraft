// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'zone_price_entry.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ZonePriceEntry _$ZonePriceEntryFromJson(Map<String, dynamic> json) =>
    ZonePriceEntry(
      id: json['id'] as String,
      unitPrice: json['unit_price'] as String,
      validFrom: DateTime.parse(json['valid_from'] as String),
      zoneCode: json['zone_code'] as String,
      zoneId: json['zone_id'] as String,
    );

Map<String, dynamic> _$ZonePriceEntryToJson(ZonePriceEntry instance) =>
    <String, dynamic>{
      'id': instance.id,
      'unit_price': instance.unitPrice,
      'valid_from': instance.validFrom.toIso8601String(),
      'zone_code': instance.zoneCode,
      'zone_id': instance.zoneId,
    };
