// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'profile_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ProfileInput _$ProfileInputFromJson(Map<String, dynamic> json) => ProfileInput(
  addressForm: json['address_form'] as String,
  title: json['title'] as String,
  language: json['language'] as String?,
);

Map<String, dynamic> _$ProfileInputToJson(ProfileInput instance) =>
    <String, dynamic>{
      'address_form': instance.addressForm,
      'language': ?instance.language,
      'title': instance.title,
    };
