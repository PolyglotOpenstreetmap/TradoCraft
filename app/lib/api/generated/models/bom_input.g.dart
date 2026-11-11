// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'bom_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

BomInput _$BomInputFromJson(Map<String, dynamic> json) => BomInput(
  lines: (json['lines'] as List<dynamic>)
      .map((e) => BomLineInput.fromJson(e as Map<String, dynamic>))
      .toList(),
  notes: json['notes'] as String?,
);

Map<String, dynamic> _$BomInputToJson(BomInput instance) => <String, dynamic>{
  'lines': instance.lines,
  'notes': ?instance.notes,
};
