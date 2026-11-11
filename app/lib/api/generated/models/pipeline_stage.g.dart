// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'pipeline_stage.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

PipelineStage _$PipelineStageFromJson(Map<String, dynamic> json) =>
    PipelineStage(
      count: (json['count'] as num).toInt(),
      stage: OpportunityStage.fromJson(json['stage'] as String),
      totalValue: json['total_value'] as String,
      weightedValue: json['weighted_value'] as String,
    );

Map<String, dynamic> _$PipelineStageToJson(PipelineStage instance) =>
    <String, dynamic>{
      'count': instance.count,
      'stage': instance.stage,
      'total_value': instance.totalValue,
      'weighted_value': instance.weightedValue,
    };
