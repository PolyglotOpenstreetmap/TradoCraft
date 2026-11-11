// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'member_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

MemberInput _$MemberInputFromJson(Map<String, dynamic> json) => MemberInput(
  userId: json['user_id'] as String,
  isLead: json['is_lead'] as bool?,
);

Map<String, dynamic> _$MemberInputToJson(MemberInput instance) =>
    <String, dynamic>{'is_lead': ?instance.isLead, 'user_id': instance.userId};
