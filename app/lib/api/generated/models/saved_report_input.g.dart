// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'saved_report_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

SavedReportInput _$SavedReportInputFromJson(Map<String, dynamic> json) =>
    SavedReportInput(
      definition: ReportDefinition.fromJson(
        json['definition'] as Map<String, dynamic>,
      ),
      name: json['name'] as String,
      description: json['description'] as String?,
      shared: json['shared'] as bool?,
    );

Map<String, dynamic> _$SavedReportInputToJson(SavedReportInput instance) =>
    <String, dynamic>{
      'definition': instance.definition,
      'description': ?instance.description,
      'name': instance.name,
      'shared': ?instance.shared,
    };
