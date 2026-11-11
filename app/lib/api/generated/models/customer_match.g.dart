// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'customer_match.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

CustomerMatch _$CustomerMatchFromJson(Map<String, dynamic> json) =>
    CustomerMatch(
      code: json['code'] as String,
      id: json['id'] as String,
      language: json['language'] as String,
      matchedOn: json['matched_on'] as String,
      name: json['name'] as String,
    );

Map<String, dynamic> _$CustomerMatchToJson(CustomerMatch instance) =>
    <String, dynamic>{
      'code': instance.code,
      'id': instance.id,
      'language': instance.language,
      'matched_on': instance.matchedOn,
      'name': instance.name,
    };
