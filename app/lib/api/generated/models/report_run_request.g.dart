// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'report_run_request.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ReportRunRequest _$ReportRunRequestFromJson(Map<String, dynamic> json) =>
    ReportRunRequest(
      definition: ReportDefinition.fromJson(
        json['definition'] as Map<String, dynamic>,
      ),
      asOf: json['as_of'] as String?,
      limit: (json['limit'] as num?)?.toInt(),
      offset: (json['offset'] as num?)?.toInt(),
    );

Map<String, dynamic> _$ReportRunRequestToJson(ReportRunRequest instance) =>
    <String, dynamic>{
      'as_of': ?instance.asOf,
      'definition': instance.definition,
      'limit': ?instance.limit,
      'offset': ?instance.offset,
    };
