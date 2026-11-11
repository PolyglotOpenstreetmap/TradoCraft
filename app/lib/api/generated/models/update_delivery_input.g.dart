// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'update_delivery_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

UpdateDeliveryInput _$UpdateDeliveryInputFromJson(Map<String, dynamic> json) =>
    UpdateDeliveryInput(
      carrier: json['carrier'] as String?,
      closureOverrideReason: json['closure_override_reason'] as String?,
      expectedUpdatedAt: json['expected_updated_at'] == null
          ? null
          : DateTime.parse(json['expected_updated_at'] as String),
      lines: (json['lines'] as List<dynamic>?)
          ?.map((e) => DeliveryLineInput.fromJson(e as Map<String, dynamic>))
          .toList(),
      notes: json['notes'] as String?,
      plannedDate: json['planned_date'] == null
          ? null
          : DateTime.parse(json['planned_date'] as String),
      shipTo: json['ship_to'] == null
          ? null
          : DeliveryShipToInput.fromJson(
              json['ship_to'] as Map<String, dynamic>,
            ),
      trackingNumber: json['tracking_number'] as String?,
    );

Map<String, dynamic> _$UpdateDeliveryInputToJson(
  UpdateDeliveryInput instance,
) => <String, dynamic>{
  'carrier': ?instance.carrier,
  'closure_override_reason': ?instance.closureOverrideReason,
  'expected_updated_at': ?instance.expectedUpdatedAt?.toIso8601String(),
  'lines': ?instance.lines,
  'notes': ?instance.notes,
  'planned_date': ?instance.plannedDate?.toIso8601String(),
  'ship_to': ?instance.shipTo,
  'tracking_number': ?instance.trackingNumber,
};
