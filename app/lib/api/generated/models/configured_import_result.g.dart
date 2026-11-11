// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'configured_import_result.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ConfiguredImportResult _$ConfiguredImportResultFromJson(
  Map<String, dynamic> json,
) => ConfiguredImportResult(
  action: json['action'] as String,
  quotation: Quotation.fromJson(json['quotation'] as Map<String, dynamic>),
);

Map<String, dynamic> _$ConfiguredImportResultToJson(
  ConfiguredImportResult instance,
) => <String, dynamic>{
  'action': instance.action,
  'quotation': instance.quotation,
};
