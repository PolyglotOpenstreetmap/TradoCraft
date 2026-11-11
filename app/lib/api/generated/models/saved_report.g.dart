// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'saved_report.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

SavedReport _$SavedReportFromJson(Map<String, dynamic> json) => SavedReport(
  createdAt: DateTime.parse(json['created_at'] as String),
  definition: ReportDefinition.fromJson(
    json['definition'] as Map<String, dynamic>,
  ),
  description: json['description'] as String,
  id: json['id'] as String,
  mine: json['mine'] as bool,
  name: json['name'] as String,
  object: json['object'] as String,
  ownerId: json['owner_id'] as String,
  ownerName: json['owner_name'] as String,
  shared: json['shared'] as bool,
  updatedAt: DateTime.parse(json['updated_at'] as String),
);

Map<String, dynamic> _$SavedReportToJson(SavedReport instance) =>
    <String, dynamic>{
      'created_at': instance.createdAt.toIso8601String(),
      'definition': instance.definition,
      'description': instance.description,
      'id': instance.id,
      'mine': instance.mine,
      'name': instance.name,
      'object': instance.object,
      'owner_id': instance.ownerId,
      'owner_name': instance.ownerName,
      'shared': instance.shared,
      'updated_at': instance.updatedAt.toIso8601String(),
    };
