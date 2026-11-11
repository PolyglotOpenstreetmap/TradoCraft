// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'issue_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

IssueInput _$IssueInputFromJson(Map<String, dynamic> json) => IssueInput(
  id: json['id'] as String,
  locationId: json['location_id'] as String,
  productId: json['product_id'] as String,
  quantity: json['quantity'] as String,
  consumeReservationLine: json['consume_reservation_line'] as String?,
  customerId: json['customer_id'] as String?,
  lotId: json['lot_id'] as String?,
  note: json['note'] as String?,
  reason: json['reason'] as String?,
  serialUnitId: json['serial_unit_id'] as String?,
  uom: json['uom'] as String?,
);

Map<String, dynamic> _$IssueInputToJson(IssueInput instance) =>
    <String, dynamic>{
      'consume_reservation_line': ?instance.consumeReservationLine,
      'customer_id': ?instance.customerId,
      'id': instance.id,
      'location_id': instance.locationId,
      'lot_id': ?instance.lotId,
      'note': ?instance.note,
      'product_id': instance.productId,
      'quantity': instance.quantity,
      'reason': ?instance.reason,
      'serial_unit_id': ?instance.serialUnitId,
      'uom': ?instance.uom,
    };
