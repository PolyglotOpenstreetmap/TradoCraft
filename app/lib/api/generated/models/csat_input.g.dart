// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'csat_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

CsatInput _$CsatInputFromJson(Map<String, dynamic> json) => CsatInput(
  rating: json['rating'] as String,
  comment: json['comment'] as String?,
);

Map<String, dynamic> _$CsatInputToJson(CsatInput instance) => <String, dynamic>{
  'comment': ?instance.comment,
  'rating': instance.rating,
};
