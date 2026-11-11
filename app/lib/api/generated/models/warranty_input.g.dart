// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'warranty_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

WarrantyInput _$WarrantyInputFromJson(Map<String, dynamic> json) =>
    WarrantyInput(
      warrantyNote: json['warranty_note'] as String?,
      warrantyUntil: json['warranty_until'] == null
          ? null
          : DateTime.parse(json['warranty_until'] as String),
    );

Map<String, dynamic> _$WarrantyInputToJson(WarrantyInput instance) =>
    <String, dynamic>{
      'warranty_note': ?instance.warrantyNote,
      'warranty_until': ?instance.warrantyUntil?.toIso8601String(),
    };
