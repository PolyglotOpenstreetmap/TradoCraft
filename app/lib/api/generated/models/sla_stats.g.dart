// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'sla_stats.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

SlaStats _$SlaStatsFromJson(Map<String, dynamic> json) => SlaStats(
  breached: (json['breached'] as num).toInt(),
  compliant: (json['compliant'] as num).toInt(),
  tickets: (json['tickets'] as num).toInt(),
  compliancePercent: (json['compliance_percent'] as num?)?.toDouble(),
);

Map<String, dynamic> _$SlaStatsToJson(SlaStats instance) => <String, dynamic>{
  'breached': instance.breached,
  'compliance_percent': ?instance.compliancePercent,
  'compliant': instance.compliant,
  'tickets': instance.tickets,
};
