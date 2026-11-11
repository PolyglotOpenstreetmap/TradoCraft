// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'portal_order.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

PortalOrder _$PortalOrderFromJson(Map<String, dynamic> json) => PortalOrder(
  canConfirm: json['can_confirm'] as bool,
  currency: json['currency'] as String,
  customerReference: json['customer_reference'] as String,
  id: json['id'] as String,
  number: json['number'] as String,
  openPriorityRequests: (json['open_priority_requests'] as num).toInt(),
  orderDate: DateTime.parse(json['order_date'] as String),
  revision: (json['revision'] as num).toInt(),
  status: json['status'] as String,
  subtotal: json['subtotal'] as String,
  total: json['total'] as String,
  vatTotal: json['vat_total'] as String,
  confirmedAt: json['confirmed_at'] == null
      ? null
      : DateTime.parse(json['confirmed_at'] as String),
);

Map<String, dynamic> _$PortalOrderToJson(PortalOrder instance) =>
    <String, dynamic>{
      'can_confirm': instance.canConfirm,
      'confirmed_at': ?instance.confirmedAt?.toIso8601String(),
      'currency': instance.currency,
      'customer_reference': instance.customerReference,
      'id': instance.id,
      'number': instance.number,
      'open_priority_requests': instance.openPriorityRequests,
      'order_date': instance.orderDate.toIso8601String(),
      'revision': instance.revision,
      'status': instance.status,
      'subtotal': instance.subtotal,
      'total': instance.total,
      'vat_total': instance.vatTotal,
    };
