// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'configuration.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Configuration _$ConfigurationFromJson(Map<String, dynamic> json) =>
    Configuration(
      createdAt: DateTime.parse(json['created_at'] as String),
      id: json['id'] as String,
      inputs: Inputs.fromJson(json['inputs'] as Map<String, dynamic>),
      modelCode: json['model_code'] as String,
      modelId: json['model_id'] as String,
      modelName: json['model_name'] as String,
      name: json['name'] as String,
      revision: (json['revision'] as num).toInt(),
      revisionId: json['revision_id'] as String,
      revisionStatus: json['revision_status'] as String,
      updatedAt: DateTime.parse(json['updated_at'] as String),
      customerId: json['customer_id'] as String?,
      customerName: json['customer_name'] as String?,
      newerRevisionId: json['newer_revision_id'] as String?,
      quotationId: json['quotation_id'] as String?,
      quotationNumber: json['quotation_number'] as String?,
      result: json['result'],
    );

Map<String, dynamic> _$ConfigurationToJson(Configuration instance) =>
    <String, dynamic>{
      'created_at': instance.createdAt.toIso8601String(),
      'customer_id': ?instance.customerId,
      'customer_name': ?instance.customerName,
      'id': instance.id,
      'inputs': instance.inputs,
      'model_code': instance.modelCode,
      'model_id': instance.modelId,
      'model_name': instance.modelName,
      'name': instance.name,
      'newer_revision_id': ?instance.newerRevisionId,
      'quotation_id': ?instance.quotationId,
      'quotation_number': ?instance.quotationNumber,
      'result': ?instance.result,
      'revision': instance.revision,
      'revision_id': instance.revisionId,
      'revision_status': instance.revisionStatus,
      'updated_at': instance.updatedAt.toIso8601String(),
    };
