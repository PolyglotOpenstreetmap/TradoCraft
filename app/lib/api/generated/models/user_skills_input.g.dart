// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'user_skills_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

UserSkillsInput _$UserSkillsInputFromJson(Map<String, dynamic> json) =>
    UserSkillsInput(
      skills: (json['skills'] as List<dynamic>)
          .map((e) => UserSkillInput.fromJson(e as Map<String, dynamic>))
          .toList(),
    );

Map<String, dynamic> _$UserSkillsInputToJson(UserSkillsInput instance) =>
    <String, dynamic>{'skills': instance.skills};
