// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'delivery_ship_to.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

DeliveryShipTo _$DeliveryShipToFromJson(Map<String, dynamic> json) =>
    DeliveryShipTo(
      city: json['city'] as String,
      countryCode: json['country_code'] as String,
      name: json['name'] as String,
      postalCode: json['postal_code'] as String,
      street: json['street'] as String,
    );

Map<String, dynamic> _$DeliveryShipToToJson(DeliveryShipTo instance) =>
    <String, dynamic>{
      'city': instance.city,
      'country_code': instance.countryCode,
      'name': instance.name,
      'postal_code': instance.postalCode,
      'street': instance.street,
    };
