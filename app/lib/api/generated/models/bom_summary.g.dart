// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'bom_summary.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

BomSummary _$BomSummaryFromJson(Map<String, dynamic> json) => BomSummary(
  createdAt: DateTime.parse(json['created_at'] as String),
  id: json['id'] as String,
  lineCount: (json['line_count'] as num).toInt(),
  notes: json['notes'] as String,
  productId: json['product_id'] as String,
  revision: (json['revision'] as num).toInt(),
  status: json['status'] as String,
  activatedAt: json['activated_at'] == null
      ? null
      : DateTime.parse(json['activated_at'] as String),
);

Map<String, dynamic> _$BomSummaryToJson(BomSummary instance) =>
    <String, dynamic>{
      'activated_at': ?instance.activatedAt?.toIso8601String(),
      'created_at': instance.createdAt.toIso8601String(),
      'id': instance.id,
      'line_count': instance.lineCount,
      'notes': instance.notes,
      'product_id': instance.productId,
      'revision': instance.revision,
      'status': instance.status,
    };
