// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'component_history.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ComponentHistory _$ComponentHistoryFromJson(Map<String, dynamic> json) =>
    ComponentHistory(
      addedAt: DateTime.parse(json['added_at'] as String),
      addedByName: json['added_by_name'] as String,
      childId: json['child_id'] as String,
      productCode: json['product_code'] as String,
      serialNumber: json['serial_number'] as String,
      removedAt: json['removed_at'] == null
          ? null
          : DateTime.parse(json['removed_at'] as String),
      removedByName: json['removed_by_name'] as String?,
    );

Map<String, dynamic> _$ComponentHistoryToJson(ComponentHistory instance) =>
    <String, dynamic>{
      'added_at': instance.addedAt.toIso8601String(),
      'added_by_name': instance.addedByName,
      'child_id': instance.childId,
      'product_code': instance.productCode,
      'removed_at': ?instance.removedAt?.toIso8601String(),
      'removed_by_name': ?instance.removedByName,
      'serial_number': instance.serialNumber,
    };
