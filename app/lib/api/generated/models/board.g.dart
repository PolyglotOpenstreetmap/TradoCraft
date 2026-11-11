// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'board.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Board _$BoardFromJson(Map<String, dynamic> json) => Board(
  backlog: (json['backlog'] as List<dynamic>)
      .map((e) => BoardJob.fromJson(e as Map<String, dynamic>))
      .toList(),
  from: DateTime.parse(json['from'] as String),
  jobs: (json['jobs'] as List<dynamic>)
      .map((e) => BoardJob.fromJson(e as Map<String, dynamic>))
      .toList(),
  settings: Settings.fromJson(json['settings'] as Map<String, dynamic>),
  skills: (json['skills'] as List<dynamic>)
      .map((e) => Skill.fromJson(e as Map<String, dynamic>))
      .toList(),
  technicians: (json['technicians'] as List<dynamic>)
      .map((e) => BoardTechnician.fromJson(e as Map<String, dynamic>))
      .toList(),
  to: DateTime.parse(json['to'] as String),
);

Map<String, dynamic> _$BoardToJson(Board instance) => <String, dynamic>{
  'backlog': instance.backlog,
  'from': instance.from.toIso8601String(),
  'jobs': instance.jobs,
  'settings': instance.settings,
  'skills': instance.skills,
  'technicians': instance.technicians,
  'to': instance.to.toIso8601String(),
};
