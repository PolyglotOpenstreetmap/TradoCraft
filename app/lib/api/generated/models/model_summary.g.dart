// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'model_summary.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ModelSummary _$ModelSummaryFromJson(Map<String, dynamic> json) => ModelSummary(
  active: json['active'] as bool,
  code: json['code'] as String,
  description: json['description'] as String,
  id: json['id'] as String,
  name: json['name'] as String,
  updatedAt: DateTime.parse(json['updated_at'] as String),
  activeRevision: (json['active_revision'] as num?)?.toInt(),
  activeRevisionId: json['active_revision_id'] as String?,
  draftRevision: (json['draft_revision'] as num?)?.toInt(),
  draftRevisionId: json['draft_revision_id'] as String?,
  productId: json['product_id'] as String?,
);

Map<String, dynamic> _$ModelSummaryToJson(ModelSummary instance) =>
    <String, dynamic>{
      'active': instance.active,
      'active_revision': ?instance.activeRevision,
      'active_revision_id': ?instance.activeRevisionId,
      'code': instance.code,
      'description': instance.description,
      'draft_revision': ?instance.draftRevision,
      'draft_revision_id': ?instance.draftRevisionId,
      'id': instance.id,
      'name': instance.name,
      'product_id': ?instance.productId,
      'updated_at': instance.updatedAt.toIso8601String(),
    };
