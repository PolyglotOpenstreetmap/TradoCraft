// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'mark_shipping_requested_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

MarkShippingRequestedInput _$MarkShippingRequestedInputFromJson(
  Map<String, dynamic> json,
) => MarkShippingRequestedInput(
  carrier: json['carrier'] as String,
  notes: json['notes'] as String?,
);

Map<String, dynamic> _$MarkShippingRequestedInputToJson(
  MarkShippingRequestedInput instance,
) => <String, dynamic>{'carrier': instance.carrier, 'notes': ?instance.notes};
