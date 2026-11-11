// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'bom.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Bom _$BomFromJson(Map<String, dynamic> json) => Bom(
  createdAt: DateTime.parse(json['created_at'] as String),
  id: json['id'] as String,
  lines: (json['lines'] as List<dynamic>)
      .map((e) => BomLine.fromJson(e as Map<String, dynamic>))
      .toList(),
  notes: json['notes'] as String,
  productCode: json['product_code'] as String,
  productId: json['product_id'] as String,
  productName: json['product_name'] as String,
  revision: (json['revision'] as num).toInt(),
  status: json['status'] as String,
  activatedAt: json['activated_at'] == null
      ? null
      : DateTime.parse(json['activated_at'] as String),
);

Map<String, dynamic> _$BomToJson(Bom instance) => <String, dynamic>{
  'activated_at': ?instance.activatedAt?.toIso8601String(),
  'created_at': instance.createdAt.toIso8601String(),
  'id': instance.id,
  'lines': instance.lines,
  'notes': instance.notes,
  'product_code': instance.productCode,
  'product_id': instance.productId,
  'product_name': instance.productName,
  'revision': instance.revision,
  'status': instance.status,
};
