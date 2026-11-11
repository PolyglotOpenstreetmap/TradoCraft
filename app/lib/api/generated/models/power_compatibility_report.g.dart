// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'power_compatibility_report.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

PowerCompatibilityReport _$PowerCompatibilityReportFromJson(
  Map<String, dynamic> json,
) => PowerCompatibilityReport(
  available: (json['available'] as List<dynamic>)
      .map((e) => PowerGrid.fromJson(e as Map<String, dynamic>))
      .toList(),
  countryCode: json['country_code'] as String,
  status: PowerStatus.fromJson(json['status'] as String),
  supported: (json['supported'] as List<dynamic>)
      .map((e) => PowerSupply.fromJson(e as Map<String, dynamic>))
      .toList(),
);

Map<String, dynamic> _$PowerCompatibilityReportToJson(
  PowerCompatibilityReport instance,
) => <String, dynamic>{
  'available': instance.available,
  'country_code': instance.countryCode,
  'status': instance.status,
  'supported': instance.supported,
};
