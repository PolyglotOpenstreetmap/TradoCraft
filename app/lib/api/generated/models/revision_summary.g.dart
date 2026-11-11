// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'revision_summary.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

RevisionSummary _$RevisionSummaryFromJson(Map<String, dynamic> json) =>
    RevisionSummary(
      createdAt: DateTime.parse(json['created_at'] as String),
      id: json['id'] as String,
      modelId: json['model_id'] as String,
      notes: json['notes'] as String,
      revision: (json['revision'] as num).toInt(),
      status: json['status'] as String,
      activatedAt: json['activated_at'] == null
          ? null
          : DateTime.parse(json['activated_at'] as String),
    );

Map<String, dynamic> _$RevisionSummaryToJson(RevisionSummary instance) =>
    <String, dynamic>{
      'activated_at': ?instance.activatedAt?.toIso8601String(),
      'created_at': instance.createdAt.toIso8601String(),
      'id': instance.id,
      'model_id': instance.modelId,
      'notes': instance.notes,
      'revision': instance.revision,
      'status': instance.status,
    };
