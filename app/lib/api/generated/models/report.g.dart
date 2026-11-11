// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'report.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Report _$ReportFromJson(Map<String, dynamic> json) => Report(
  backlogByOwner: (json['backlog_by_owner'] as List<dynamic>)
      .map((e) => CountRow.fromJson(e as Map<String, dynamic>))
      .toList(),
  backlogByPriority: (json['backlog_by_priority'] as List<dynamic>)
      .map((e) => CountRow.fromJson(e as Map<String, dynamic>))
      .toList(),
  backlogByStatus: (json['backlog_by_status'] as List<dynamic>)
      .map((e) => CountRow.fromJson(e as Map<String, dynamic>))
      .toList(),
  backlogByTeam: (json['backlog_by_team'] as List<dynamic>)
      .map((e) => CountRow.fromJson(e as Map<String, dynamic>))
      .toList(),
  basis: json['basis'] as String,
  breachesByTeam: (json['breaches_by_team'] as List<dynamic>)
      .map((e) => TeamBreaches.fromJson(e as Map<String, dynamic>))
      .toList(),
  csat: CsatStats.fromJson(json['csat'] as Map<String, dynamic>),
  firstResponse: DurationStats.fromJson(
    json['first_response'] as Map<String, dynamic>,
  ),
  from: DateTime.parse(json['from'] as String),
  group: json['group'] as String,
  resolution: DurationStats.fromJson(
    json['resolution'] as Map<String, dynamic>,
  ),
  sla: SlaStats.fromJson(json['sla'] as Map<String, dynamic>),
  timeBillableMinutes: (json['time_billable_minutes'] as num).toInt(),
  timeByUser: (json['time_by_user'] as List<dynamic>)
      .map((e) => TimeRow.fromJson(e as Map<String, dynamic>))
      .toList(),
  timeNonBillableMinutes: (json['time_non_billable_minutes'] as num).toInt(),
  to: DateTime.parse(json['to'] as String),
  topCategories: (json['top_categories'] as List<dynamic>)
      .map((e) => CountRow.fromJson(e as Map<String, dynamic>))
      .toList(),
  topTags: (json['top_tags'] as List<dynamic>)
      .map((e) => CountRow.fromJson(e as Map<String, dynamic>))
      .toList(),
  volume: (json['volume'] as List<dynamic>)
      .map((e) => VolumePoint.fromJson(e as Map<String, dynamic>))
      .toList(),
);

Map<String, dynamic> _$ReportToJson(Report instance) => <String, dynamic>{
  'backlog_by_owner': instance.backlogByOwner,
  'backlog_by_priority': instance.backlogByPriority,
  'backlog_by_status': instance.backlogByStatus,
  'backlog_by_team': instance.backlogByTeam,
  'basis': instance.basis,
  'breaches_by_team': instance.breachesByTeam,
  'csat': instance.csat,
  'first_response': instance.firstResponse,
  'from': instance.from.toIso8601String(),
  'group': instance.group,
  'resolution': instance.resolution,
  'sla': instance.sla,
  'time_billable_minutes': instance.timeBillableMinutes,
  'time_by_user': instance.timeByUser,
  'time_non_billable_minutes': instance.timeNonBillableMinutes,
  'to': instance.to.toIso8601String(),
  'top_categories': instance.topCategories,
  'top_tags': instance.topTags,
  'volume': instance.volume,
};
