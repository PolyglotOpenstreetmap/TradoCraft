// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'job_skills_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

JobSkillsInput _$JobSkillsInputFromJson(Map<String, dynamic> json) =>
    JobSkillsInput(
      skillIds: (json['skill_ids'] as List<dynamic>)
          .map((e) => e as String)
          .toList(),
    );

Map<String, dynamic> _$JobSkillsInputToJson(JobSkillsInput instance) =>
    <String, dynamic>{'skill_ids': instance.skillIds};
