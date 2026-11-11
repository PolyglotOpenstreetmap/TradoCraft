// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'contact_phone.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ContactPhone _$ContactPhoneFromJson(Map<String, dynamic> json) => ContactPhone(
  label: json['label'] as String,
  normalized: json['normalized'] as String,
  number: json['number'] as String,
);

Map<String, dynamic> _$ContactPhoneToJson(ContactPhone instance) =>
    <String, dynamic>{
      'label': instance.label,
      'normalized': instance.normalized,
      'number': instance.number,
    };
