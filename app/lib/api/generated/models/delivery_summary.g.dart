// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'delivery_summary.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

DeliverySummary _$DeliverySummaryFromJson(Map<String, dynamic> json) =>
    DeliverySummary(
      carrier: json['carrier'] as String,
      createdAt: DateTime.parse(json['created_at'] as String),
      customerId: json['customer_id'] as String,
      customerName: json['customer_name'] as String,
      id: json['id'] as String,
      lineCount: (json['line_count'] as num).toInt(),
      number: json['number'] as String,
      orderId: json['order_id'] as String,
      orderNumber: json['order_number'] as String,
      plannedOnClosure: json['planned_on_closure'] as bool,
      status: json['status'] as String,
      trackingNumber: json['tracking_number'] as String,
      plannedDate: json['planned_date'] == null
          ? null
          : DateTime.parse(json['planned_date'] as String),
      shippedAt: json['shipped_at'] == null
          ? null
          : DateTime.parse(json['shipped_at'] as String),
    );

Map<String, dynamic> _$DeliverySummaryToJson(DeliverySummary instance) =>
    <String, dynamic>{
      'carrier': instance.carrier,
      'created_at': instance.createdAt.toIso8601String(),
      'customer_id': instance.customerId,
      'customer_name': instance.customerName,
      'id': instance.id,
      'line_count': instance.lineCount,
      'number': instance.number,
      'order_id': instance.orderId,
      'order_number': instance.orderNumber,
      'planned_date': ?instance.plannedDate?.toIso8601String(),
      'planned_on_closure': instance.plannedOnClosure,
      'shipped_at': ?instance.shippedAt?.toIso8601String(),
      'status': instance.status,
      'tracking_number': instance.trackingNumber,
    };
