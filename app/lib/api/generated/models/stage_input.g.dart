// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'stage_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

StageInput _$StageInputFromJson(Map<String, dynamic> json) => StageInput(
  stage: OpportunityStage.fromJson(json['stage'] as String),
  lostReason: json['lost_reason'] as String?,
  probability: (json['probability'] as num?)?.toInt(),
);

Map<String, dynamic> _$StageInputToJson(StageInput instance) =>
    <String, dynamic>{
      'lost_reason': ?instance.lostReason,
      'probability': ?instance.probability,
      'stage': instance.stage,
    };
