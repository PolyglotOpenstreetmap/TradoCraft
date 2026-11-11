// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'delivery_ship_to_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

DeliveryShipToInput _$DeliveryShipToInputFromJson(Map<String, dynamic> json) =>
    DeliveryShipToInput(
      countryCode: json['country_code'] as String,
      name: json['name'] as String,
      city: json['city'] as String?,
      postalCode: json['postal_code'] as String?,
      street: json['street'] as String?,
    );

Map<String, dynamic> _$DeliveryShipToInputToJson(
  DeliveryShipToInput instance,
) => <String, dynamic>{
  'city': ?instance.city,
  'country_code': instance.countryCode,
  'name': instance.name,
  'postal_code': ?instance.postalCode,
  'street': ?instance.street,
};
