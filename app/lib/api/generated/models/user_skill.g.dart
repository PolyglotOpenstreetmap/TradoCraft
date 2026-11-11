// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'user_skill.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

UserSkill _$UserSkillFromJson(Map<String, dynamic> json) => UserSkill(
  code: json['code'] as String,
  level: (json['level'] as num).toInt(),
  name: json['name'] as String,
  skillId: json['skill_id'] as String,
);

Map<String, dynamic> _$UserSkillToJson(UserSkill instance) => <String, dynamic>{
  'code': instance.code,
  'level': instance.level,
  'name': instance.name,
  'skill_id': instance.skillId,
};
