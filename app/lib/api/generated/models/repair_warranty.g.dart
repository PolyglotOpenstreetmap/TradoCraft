// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'repair_warranty.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

RepairWarranty _$RepairWarrantyFromJson(Map<String, dynamic> json) =>
    RepairWarranty(
      arrived: json['arrived'] as bool,
      auto: json['auto'] as String,
      candidates: (json['candidates'] as List<dynamic>)
          .map((e) => SerialUnitRef.fromJson(e as Map<String, dynamic>))
          .toList(),
      comparedWith: DateTime.parse(json['compared_with'] as String),
      overridden: json['overridden'] as bool,
      overrideReason: json['override_reason'] as String,
      repairId: json['repair_id'] as String,
      warranty: json['warranty'] as String,
      autoAt: json['auto_at'] == null
          ? null
          : DateTime.parse(json['auto_at'] as String),
      autoUntil: json['auto_until'] == null
          ? null
          : DateTime.parse(json['auto_until'] as String),
      overrideAt: json['override_at'] == null
          ? null
          : DateTime.parse(json['override_at'] as String),
      overrideBy: json['override_by'] as String?,
      overrideByName: json['override_by_name'] as String?,
      serialUnit: json['serial_unit'] == null
          ? null
          : SerialUnitRef.fromJson(json['serial_unit'] as Map<String, dynamic>),
    );

Map<String, dynamic> _$RepairWarrantyToJson(RepairWarranty instance) =>
    <String, dynamic>{
      'arrived': instance.arrived,
      'auto': instance.auto,
      'auto_at': ?instance.autoAt?.toIso8601String(),
      'auto_until': ?instance.autoUntil?.toIso8601String(),
      'candidates': instance.candidates,
      'compared_with': instance.comparedWith.toIso8601String(),
      'overridden': instance.overridden,
      'override_at': ?instance.overrideAt?.toIso8601String(),
      'override_by': ?instance.overrideBy,
      'override_by_name': ?instance.overrideByName,
      'override_reason': instance.overrideReason,
      'repair_id': instance.repairId,
      'serial_unit': ?instance.serialUnit,
      'warranty': instance.warranty,
    };
