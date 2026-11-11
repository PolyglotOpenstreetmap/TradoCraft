// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'inputs.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Inputs _$InputsFromJson(Map<String, dynamic> json) => Inputs(
  groups: json['groups'],
  items: json['items'],
  pins: (json['pins'] as List<dynamic>?)
      ?.map((e) => Pin.fromJson(e as Map<String, dynamic>))
      .toList(),
  project: json['project'],
);

Map<String, dynamic> _$InputsToJson(Inputs instance) => <String, dynamic>{
  'groups': ?instance.groups,
  'items': ?instance.items,
  'pins': ?instance.pins,
  'project': ?instance.project,
};
