// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'revision.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Revision _$RevisionFromJson(Map<String, dynamic> json) => Revision(
  createdAt: DateTime.parse(json['created_at'] as String),
  definition: json['definition'],
  errors: (json['errors'] as List<dynamic>)
      .map((e) => DefinitionError.fromJson(e as Map<String, dynamic>))
      .toList(),
  id: json['id'] as String,
  modelCode: json['model_code'] as String,
  modelId: json['model_id'] as String,
  modelName: json['model_name'] as String,
  notes: json['notes'] as String,
  revision: (json['revision'] as num).toInt(),
  status: json['status'] as String,
  updatedAt: DateTime.parse(json['updated_at'] as String),
  activatedAt: json['activated_at'] == null
      ? null
      : DateTime.parse(json['activated_at'] as String),
);

Map<String, dynamic> _$RevisionToJson(Revision instance) => <String, dynamic>{
  'activated_at': ?instance.activatedAt?.toIso8601String(),
  'created_at': instance.createdAt.toIso8601String(),
  'definition': instance.definition,
  'errors': instance.errors,
  'id': instance.id,
  'model_code': instance.modelCode,
  'model_id': instance.modelId,
  'model_name': instance.modelName,
  'notes': instance.notes,
  'revision': instance.revision,
  'status': instance.status,
  'updated_at': instance.updatedAt.toIso8601String(),
};
