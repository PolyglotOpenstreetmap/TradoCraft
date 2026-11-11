// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'create_delivery_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

CreateDeliveryInput _$CreateDeliveryInputFromJson(Map<String, dynamic> json) =>
    CreateDeliveryInput(
      carrier: json['carrier'] as String?,
      closureOverrideReason: json['closure_override_reason'] as String?,
      notes: json['notes'] as String?,
      plannedDate: json['planned_date'] == null
          ? null
          : DateTime.parse(json['planned_date'] as String),
    );

Map<String, dynamic> _$CreateDeliveryInputToJson(
  CreateDeliveryInput instance,
) => <String, dynamic>{
  'carrier': ?instance.carrier,
  'closure_override_reason': ?instance.closureOverrideReason,
  'notes': ?instance.notes,
  'planned_date': ?instance.plannedDate?.toIso8601String(),
};
