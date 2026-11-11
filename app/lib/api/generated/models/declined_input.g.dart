// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'declined_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

DeclinedInput _$DeclinedInputFromJson(Map<String, dynamic> json) =>
    DeclinedInput(
      reason: json['reason'] as String,
      signerName: json['signer_name'] as String?,
    );

Map<String, dynamic> _$DeclinedInputToJson(DeclinedInput instance) =>
    <String, dynamic>{
      'reason': instance.reason,
      'signer_name': ?instance.signerName,
    };
