// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'lot_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

LotInput _$LotInputFromJson(Map<String, dynamic> json) => LotInput(
  expiryDate: json['expiry_date'] == null
      ? null
      : DateTime.parse(json['expiry_date'] as String),
  notes: json['notes'] as String?,
);

Map<String, dynamic> _$LotInputToJson(LotInput instance) => <String, dynamic>{
  'expiry_date': ?instance.expiryDate?.toIso8601String(),
  'notes': ?instance.notes,
};
