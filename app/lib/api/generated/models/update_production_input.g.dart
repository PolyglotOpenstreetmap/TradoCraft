// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'update_production_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

UpdateProductionInput _$UpdateProductionInputFromJson(
  Map<String, dynamic> json,
) => UpdateProductionInput(
  divisionId: json['division_id'] as String?,
  notes: json['notes'] as String?,
  plannedReadyDate: json['planned_ready_date'] == null
      ? null
      : DateTime.parse(json['planned_ready_date'] as String),
);

Map<String, dynamic> _$UpdateProductionInputToJson(
  UpdateProductionInput instance,
) => <String, dynamic>{
  'division_id': ?instance.divisionId,
  'notes': ?instance.notes,
  'planned_ready_date': ?instance.plannedReadyDate?.toIso8601String(),
};
