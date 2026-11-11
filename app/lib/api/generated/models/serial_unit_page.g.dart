// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'serial_unit_page.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

SerialUnitPage _$SerialUnitPageFromJson(Map<String, dynamic> json) =>
    SerialUnitPage(
      items: (json['items'] as List<dynamic>)
          .map((e) => SerialUnit.fromJson(e as Map<String, dynamic>))
          .toList(),
      total: (json['total'] as num).toInt(),
    );

Map<String, dynamic> _$SerialUnitPageToJson(SerialUnitPage instance) =>
    <String, dynamic>{'items': instance.items, 'total': instance.total};
