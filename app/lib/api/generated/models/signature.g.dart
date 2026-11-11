// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'signature.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Signature _$SignatureFromJson(Map<String, dynamic> json) => Signature(
  description: json['description'] as String,
  group: json['group'] as String,
  name: json['name'] as String,
  usage: json['usage'] as String,
);

Map<String, dynamic> _$SignatureToJson(Signature instance) => <String, dynamic>{
  'description': instance.description,
  'group': instance.group,
  'name': instance.name,
  'usage': instance.usage,
};
