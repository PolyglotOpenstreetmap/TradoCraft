// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'team_member.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

TeamMember _$TeamMemberFromJson(Map<String, dynamic> json) => TeamMember(
  displayName: json['display_name'] as String,
  isLead: json['is_lead'] as bool,
  userId: json['user_id'] as String,
);

Map<String, dynamic> _$TeamMemberToJson(TeamMember instance) =>
    <String, dynamic>{
      'display_name': instance.displayName,
      'is_lead': instance.isLead,
      'user_id': instance.userId,
    };
