// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'user_skill_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

UserSkillInput _$UserSkillInputFromJson(Map<String, dynamic> json) =>
    UserSkillInput(
      level: (json['level'] as num).toInt(),
      skillId: json['skill_id'] as String,
    );

Map<String, dynamic> _$UserSkillInputToJson(UserSkillInput instance) =>
    <String, dynamic>{'level': instance.level, 'skill_id': instance.skillId};
