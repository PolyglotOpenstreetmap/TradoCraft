// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'ship_delivery_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ShipDeliveryInput _$ShipDeliveryInputFromJson(Map<String, dynamic> json) =>
    ShipDeliveryInput(
      carrier: json['carrier'] as String?,
      trackingNumber: json['tracking_number'] as String?,
    );

Map<String, dynamic> _$ShipDeliveryInputToJson(ShipDeliveryInput instance) =>
    <String, dynamic>{
      'carrier': ?instance.carrier,
      'tracking_number': ?instance.trackingNumber,
    };
