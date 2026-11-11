// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'sla_policies_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

SlaPoliciesInput _$SlaPoliciesInputFromJson(Map<String, dynamic> json) =>
    SlaPoliciesInput(
      policies: (json['policies'] as List<dynamic>)
          .map((e) => SlaPolicy.fromJson(e as Map<String, dynamic>))
          .toList(),
    );

Map<String, dynamic> _$SlaPoliciesInputToJson(SlaPoliciesInput instance) =>
    <String, dynamic>{'policies': instance.policies};
