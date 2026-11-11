// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'change.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Change _$ChangeFromJson(Map<String, dynamic> json) => Change(
  kind: json['kind'] as String,
  from: json['from'] as String?,
  to: json['to'] as String?,
);

Map<String, dynamic> _$ChangeToJson(Change instance) => <String, dynamic>{
  'from': ?instance.from,
  'kind': instance.kind,
  'to': ?instance.to,
};
