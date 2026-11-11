// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'zone_price_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ZonePriceInput _$ZonePriceInputFromJson(Map<String, dynamic> json) =>
    ZonePriceInput(
      unitPrice: json['unit_price'] as String,
      validFrom: DateTime.parse(json['valid_from'] as String),
      zoneId: json['zone_id'] as String,
    );

Map<String, dynamic> _$ZonePriceInputToJson(ZonePriceInput instance) =>
    <String, dynamic>{
      'unit_price': instance.unitPrice,
      'valid_from': instance.validFrom.toIso8601String(),
      'zone_id': instance.zoneId,
    };
