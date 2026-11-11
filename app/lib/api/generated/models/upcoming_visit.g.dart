// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'upcoming_visit.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

UpcomingVisit _$UpcomingVisitFromJson(Map<String, dynamic> json) =>
    UpcomingVisit(
      dueDate: DateTime.parse(json['due_date'] as String),
      generateOn: DateTime.parse(json['generate_on'] as String),
      kind: json['kind'] as String,
      templateId: json['template_id'] as String,
      title: json['title'] as String,
      jobId: json['job_id'] as String?,
      jobNumber: json['job_number'] as String?,
    );

Map<String, dynamic> _$UpcomingVisitToJson(UpcomingVisit instance) =>
    <String, dynamic>{
      'due_date': instance.dueDate.toIso8601String(),
      'generate_on': instance.generateOn.toIso8601String(),
      'job_id': ?instance.jobId,
      'job_number': ?instance.jobNumber,
      'kind': instance.kind,
      'template_id': instance.templateId,
      'title': instance.title,
    };
