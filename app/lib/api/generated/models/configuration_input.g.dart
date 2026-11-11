// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'configuration_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ConfigurationInput _$ConfigurationInputFromJson(Map<String, dynamic> json) =>
    ConfigurationInput(
      customerId: json['customer_id'] as String?,
      inputs: json['inputs'] == null
          ? null
          : Inputs.fromJson(json['inputs'] as Map<String, dynamic>),
      modelId: json['model_id'] as String?,
      name: json['name'] as String?,
      revisionId: json['revision_id'] as String?,
    );

Map<String, dynamic> _$ConfigurationInputToJson(ConfigurationInput instance) =>
    <String, dynamic>{
      'customer_id': ?instance.customerId,
      'inputs': ?instance.inputs,
      'model_id': ?instance.modelId,
      'name': ?instance.name,
      'revision_id': ?instance.revisionId,
    };
