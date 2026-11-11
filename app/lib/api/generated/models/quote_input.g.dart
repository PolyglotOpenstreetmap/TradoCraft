// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'quote_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

QuoteInput _$QuoteInputFromJson(Map<String, dynamic> json) => QuoteInput(
  notes: json['notes'] as String?,
  purpose: json['purpose'] as String?,
  validDays: (json['valid_days'] as num?)?.toInt(),
);

Map<String, dynamic> _$QuoteInputToJson(QuoteInput instance) =>
    <String, dynamic>{
      'notes': ?instance.notes,
      'purpose': ?instance.purpose,
      'valid_days': ?instance.validDays,
    };
