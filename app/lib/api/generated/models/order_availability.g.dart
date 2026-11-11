// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'order_availability.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

OrderAvailability _$OrderAvailabilityFromJson(Map<String, dynamic> json) =>
    OrderAvailability(
      lines: (json['lines'] as List<dynamic>)
          .map((e) => LineAvailability.fromJson(e as Map<String, dynamic>))
          .toList(),
      orderId: json['order_id'] as String,
      orderNumber: json['order_number'] as String,
      orderStatus: json['order_status'] as String,
      reservable: json['reservable'] as bool,
    );

Map<String, dynamic> _$OrderAvailabilityToJson(OrderAvailability instance) =>
    <String, dynamic>{
      'lines': instance.lines,
      'order_id': instance.orderId,
      'order_number': instance.orderNumber,
      'order_status': instance.orderStatus,
      'reservable': instance.reservable,
    };
