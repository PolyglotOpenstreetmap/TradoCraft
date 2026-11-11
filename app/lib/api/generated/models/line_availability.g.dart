// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'line_availability.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

LineAvailability _$LineAvailabilityFromJson(Map<String, dynamic> json) =>
    LineAvailability(
      available: json['available'] as String,
      consumed: json['consumed'] as String,
      description: json['description'] as String,
      expired: json['expired'] as String,
      onHand: json['on_hand'] as String,
      orderLineId: json['order_line_id'] as String,
      ordered: json['ordered'] as String,
      orderedStock: json['ordered_stock'] as String,
      position: (json['position'] as num).toInt(),
      productCode: json['product_code'] as String,
      reserved: json['reserved'] as String,
      reservedTotal: json['reserved_total'] as String,
      shortfall: json['shortfall'] as String,
      state: json['state'] as String,
      unit: json['unit'] as String,
      productId: json['product_id'] as String?,
    );

Map<String, dynamic> _$LineAvailabilityToJson(LineAvailability instance) =>
    <String, dynamic>{
      'available': instance.available,
      'consumed': instance.consumed,
      'description': instance.description,
      'expired': instance.expired,
      'on_hand': instance.onHand,
      'order_line_id': instance.orderLineId,
      'ordered': instance.ordered,
      'ordered_stock': instance.orderedStock,
      'position': instance.position,
      'product_code': instance.productCode,
      'product_id': ?instance.productId,
      'reserved': instance.reserved,
      'reserved_total': instance.reservedTotal,
      'shortfall': instance.shortfall,
      'state': instance.state,
      'unit': instance.unit,
    };
