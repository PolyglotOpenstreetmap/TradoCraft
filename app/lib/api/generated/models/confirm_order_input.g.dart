// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'confirm_order_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ConfirmOrderInput _$ConfirmOrderInputFromJson(Map<String, dynamic> json) =>
    ConfirmOrderInput(
      note: json['note'] as String?,
      revision: (json['revision'] as num?)?.toInt(),
    );

Map<String, dynamic> _$ConfirmOrderInputToJson(ConfirmOrderInput instance) =>
    <String, dynamic>{'note': ?instance.note, 'revision': ?instance.revision};
