// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'serial_component.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

SerialComponent _$SerialComponentFromJson(Map<String, dynamic> json) =>
    SerialComponent(
      addedAt: DateTime.parse(json['added_at'] as String),
      id: json['id'] as String,
      level: (json['level'] as num).toInt(),
      parentId: json['parent_id'] as String,
      productCode: json['product_code'] as String,
      productName: json['product_name'] as String,
      serialNumber: json['serial_number'] as String,
      status: json['status'] as String,
    );

Map<String, dynamic> _$SerialComponentToJson(SerialComponent instance) =>
    <String, dynamic>{
      'added_at': instance.addedAt.toIso8601String(),
      'id': instance.id,
      'level': instance.level,
      'parent_id': instance.parentId,
      'product_code': instance.productCode,
      'product_name': instance.productName,
      'serial_number': instance.serialNumber,
      'status': instance.status,
    };
