// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'line_reservation.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

LineReservation _$LineReservationFromJson(Map<String, dynamic> json) =>
    LineReservation(
      allocations: (json['allocations'] as List<dynamic>)
          .map((e) => Allocation.fromJson(e as Map<String, dynamic>))
          .toList(),
      consumed: json['consumed'] as String,
      orderLineId: json['order_line_id'] as String,
      ordered: json['ordered'] as String,
      productCode: json['product_code'] as String,
      productId: json['product_id'] as String,
      reserved: json['reserved'] as String,
    );

Map<String, dynamic> _$LineReservationToJson(LineReservation instance) =>
    <String, dynamic>{
      'allocations': instance.allocations,
      'consumed': instance.consumed,
      'order_line_id': instance.orderLineId,
      'ordered': instance.ordered,
      'product_code': instance.productCode,
      'product_id': instance.productId,
      'reserved': instance.reserved,
    };
