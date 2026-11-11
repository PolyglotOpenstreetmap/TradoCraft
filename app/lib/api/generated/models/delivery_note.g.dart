// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'delivery_note.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

DeliveryNote _$DeliveryNoteFromJson(Map<String, dynamic> json) => DeliveryNote(
  carrier: json['carrier'] as String,
  createdAt: DateTime.parse(json['created_at'] as String),
  customerId: json['customer_id'] as String,
  customerName: json['customer_name'] as String,
  id: json['id'] as String,
  lines: (json['lines'] as List<dynamic>)
      .map((e) => DeliveryNoteLine.fromJson(e as Map<String, dynamic>))
      .toList(),
  notes: json['notes'] as String,
  number: json['number'] as String,
  orderId: json['order_id'] as String,
  orderNumber: json['order_number'] as String,
  orderRevision: (json['order_revision'] as num).toInt(),
  plannedOnClosure: json['planned_on_closure'] as bool,
  shipTo: DeliveryShipTo.fromJson(json['ship_to'] as Map<String, dynamic>),
  status: json['status'] as String,
  trackingNumber: json['tracking_number'] as String,
  updatedAt: DateTime.parse(json['updated_at'] as String),
  closureOverrideReason: json['closure_override_reason'] as String?,
  closureWarning: json['closure_warning'] == null
      ? null
      : ClosureWarning.fromJson(
          json['closure_warning'] as Map<String, dynamic>,
        ),
  plannedDate: json['planned_date'] == null
      ? null
      : DateTime.parse(json['planned_date'] as String),
  shippedAt: json['shipped_at'] == null
      ? null
      : DateTime.parse(json['shipped_at'] as String),
);

Map<String, dynamic> _$DeliveryNoteToJson(DeliveryNote instance) =>
    <String, dynamic>{
      'carrier': instance.carrier,
      'closure_override_reason': ?instance.closureOverrideReason,
      'closure_warning': ?instance.closureWarning,
      'created_at': instance.createdAt.toIso8601String(),
      'customer_id': instance.customerId,
      'customer_name': instance.customerName,
      'id': instance.id,
      'lines': instance.lines,
      'notes': instance.notes,
      'number': instance.number,
      'order_id': instance.orderId,
      'order_number': instance.orderNumber,
      'order_revision': instance.orderRevision,
      'planned_date': ?instance.plannedDate?.toIso8601String(),
      'planned_on_closure': instance.plannedOnClosure,
      'ship_to': instance.shipTo,
      'shipped_at': ?instance.shippedAt?.toIso8601String(),
      'status': instance.status,
      'tracking_number': instance.trackingNumber,
      'updated_at': instance.updatedAt.toIso8601String(),
    };
