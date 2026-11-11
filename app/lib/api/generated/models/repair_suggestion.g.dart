// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'repair_suggestion.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

RepairSuggestion _$RepairSuggestionFromJson(Map<String, dynamic> json) =>
    RepairSuggestion(
      reasons: (json['reasons'] as List<dynamic>)
          .map((e) => e as String)
          .toList(),
      repair: Repair.fromJson(json['repair'] as Map<String, dynamic>),
      score: (json['score'] as num).toInt(),
    );

Map<String, dynamic> _$RepairSuggestionToJson(RepairSuggestion instance) =>
    <String, dynamic>{
      'reasons': instance.reasons,
      'repair': instance.repair,
      'score': instance.score,
    };
