// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'job_detail.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

JobDetail _$JobDetailFromJson(Map<String, dynamic> json) => JobDetail(
  contactPhones: (json['contact_phones'] as List<dynamic>)
      .map((e) => SitePhone.fromJson(e as Map<String, dynamic>))
      .toList(),
  equipment: (json['equipment'] as List<dynamic>)
      .map((e) => JobEquipment.fromJson(e as Map<String, dynamic>))
      .toList(),
  job: Job.fromJson(json['job'] as Map<String, dynamic>),
  parts: (json['parts'] as List<dynamic>)
      .map((e) => JobPart.fromJson(e as Map<String, dynamic>))
      .toList(),
  photos: (json['photos'] as List<dynamic>)
      .map((e) => JobPhoto.fromJson(e as Map<String, dynamic>))
      .toList(),
  tasks: (json['tasks'] as List<dynamic>)
      .map((e) => Task.fromJson(e as Map<String, dynamic>))
      .toList(),
  travelMinutes: (json['travel_minutes'] as num).toInt(),
  workLogs: (json['work_logs'] as List<dynamic>)
      .map((e) => WorkLog.fromJson(e as Map<String, dynamic>))
      .toList(),
  workMinutes: (json['work_minutes'] as num).toInt(),
  signature: json['signature'] == null
      ? null
      : JobSignature.fromJson(json['signature'] as Map<String, dynamic>),
);

Map<String, dynamic> _$JobDetailToJson(JobDetail instance) => <String, dynamic>{
  'contact_phones': instance.contactPhones,
  'equipment': instance.equipment,
  'job': instance.job,
  'parts': instance.parts,
  'photos': instance.photos,
  'signature': ?instance.signature,
  'tasks': instance.tasks,
  'travel_minutes': instance.travelMinutes,
  'work_logs': instance.workLogs,
  'work_minutes': instance.workMinutes,
};
