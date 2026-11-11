// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'delegate_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

DelegateInput _$DelegateInputFromJson(Map<String, dynamic> json) =>
    DelegateInput(
      toUserId: json['to_user_id'] as String,
      comment: json['comment'] as String?,
    );

Map<String, dynamic> _$DelegateInputToJson(DelegateInput instance) =>
    <String, dynamic>{
      'comment': ?instance.comment,
      'to_user_id': instance.toUserId,
    };
