// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'configuration_summary.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ConfigurationSummary _$ConfigurationSummaryFromJson(
  Map<String, dynamic> json,
) => ConfigurationSummary(
  id: json['id'] as String,
  modelCode: json['model_code'] as String,
  modelId: json['model_id'] as String,
  modelName: json['model_name'] as String,
  name: json['name'] as String,
  revision: (json['revision'] as num).toInt(),
  updatedAt: DateTime.parse(json['updated_at'] as String),
  customerId: json['customer_id'] as String?,
  customerName: json['customer_name'] as String?,
  newerRevision: (json['newer_revision'] as num?)?.toInt(),
  quotationId: json['quotation_id'] as String?,
  quotationNumber: json['quotation_number'] as String?,
);

Map<String, dynamic> _$ConfigurationSummaryToJson(
  ConfigurationSummary instance,
) => <String, dynamic>{
  'customer_id': ?instance.customerId,
  'customer_name': ?instance.customerName,
  'id': instance.id,
  'model_code': instance.modelCode,
  'model_id': instance.modelId,
  'model_name': instance.modelName,
  'name': instance.name,
  'newer_revision': ?instance.newerRevision,
  'quotation_id': ?instance.quotationId,
  'quotation_number': ?instance.quotationNumber,
  'revision': instance.revision,
  'updated_at': instance.updatedAt.toIso8601String(),
};
