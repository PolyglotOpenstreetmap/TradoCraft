// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'board_technician.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

BoardTechnician _$BoardTechnicianFromJson(Map<String, dynamic> json) =>
    BoardTechnician(
      absences: (json['absences'] as List<dynamic>)
          .map((e) => Absence.fromJson(e as Map<String, dynamic>))
          .toList(),
      displayName: json['display_name'] as String,
      hoursAreDefault: json['hours_are_default'] as bool,
      skills: (json['skills'] as List<dynamic>)
          .map((e) => UserSkill.fromJson(e as Map<String, dynamic>))
          .toList(),
      userId: json['user_id'] as String,
      workingHours: (json['working_hours'] as List<dynamic>)
          .map((e) => TechnicianHours.fromJson(e as Map<String, dynamic>))
          .toList(),
    );

Map<String, dynamic> _$BoardTechnicianToJson(BoardTechnician instance) =>
    <String, dynamic>{
      'absences': instance.absences,
      'display_name': instance.displayName,
      'hours_are_default': instance.hoursAreDefault,
      'skills': instance.skills,
      'user_id': instance.userId,
      'working_hours': instance.workingHours,
    };
