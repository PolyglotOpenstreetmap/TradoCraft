// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'contract_summary.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ContractSummary _$ContractSummaryFromJson(Map<String, dynamic> json) =>
    ContractSummary(
      autoRenew: json['auto_renew'] as bool,
      customerId: json['customer_id'] as String,
      customerName: json['customer_name'] as String,
      id: json['id'] as String,
      number: json['number'] as String,
      openJobs: (json['open_jobs'] as num).toInt(),
      startDate: DateTime.parse(json['start_date'] as String),
      status: json['status'] as String,
      title: json['title'] as String,
      endDate: json['end_date'] == null
          ? null
          : DateTime.parse(json['end_date'] as String),
      resolutionHours: (json['resolution_hours'] as num?)?.toInt(),
      responseHours: (json['response_hours'] as num?)?.toInt(),
    );

Map<String, dynamic> _$ContractSummaryToJson(ContractSummary instance) =>
    <String, dynamic>{
      'auto_renew': instance.autoRenew,
      'customer_id': instance.customerId,
      'customer_name': instance.customerName,
      'end_date': ?instance.endDate?.toIso8601String(),
      'id': instance.id,
      'number': instance.number,
      'open_jobs': instance.openJobs,
      'resolution_hours': ?instance.resolutionHours,
      'response_hours': ?instance.responseHours,
      'start_date': instance.startDate.toIso8601String(),
      'status': instance.status,
      'title': instance.title,
    };
