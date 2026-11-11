// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'serial_unit_detail.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

SerialUnitDetail _$SerialUnitDetailFromJson(Map<String, dynamic> json) =>
    SerialUnitDetail(
      components: (json['components'] as List<dynamic>)
          .map((e) => SerialComponent.fromJson(e as Map<String, dynamic>))
          .toList(),
      history: (json['history'] as List<dynamic>)
          .map((e) => ComponentHistory.fromJson(e as Map<String, dynamic>))
          .toList(),
      unit: SerialUnit.fromJson(json['unit'] as Map<String, dynamic>),
    );

Map<String, dynamic> _$SerialUnitDetailToJson(SerialUnitDetail instance) =>
    <String, dynamic>{
      'components': instance.components,
      'history': instance.history,
      'unit': instance.unit,
    };
