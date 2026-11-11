// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'production_order.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ProductionOrder _$ProductionOrderFromJson(Map<String, dynamic> json) =>
    ProductionOrder(
      bomRevision: (json['bom_revision'] as num).toInt(),
      createdAt: DateTime.parse(json['created_at'] as String),
      id: json['id'] as String,
      number: json['number'] as String,
      orderCancelled: json['order_cancelled'] as bool,
      priority: json['priority'] as String,
      productCode: json['product_code'] as String,
      productId: json['product_id'] as String,
      productName: json['product_name'] as String,
      productTracking: json['product_tracking'] as String,
      quantity: json['quantity'] as String,
      shortComponents: (json['short_components'] as num).toInt(),
      status: json['status'] as String,
      unit: json['unit'] as String,
      components: (json['components'] as List<dynamic>)
          .map((e) => ProductionComponent.fromJson(e as Map<String, dynamic>))
          .toList(),
      notes: json['notes'] as String,
      outputs: (json['outputs'] as List<dynamic>)
          .map((e) => ProductionOutput.fromJson(e as Map<String, dynamic>))
          .toList(),
      customerName: json['customer_name'] as String?,
      divisionId: json['division_id'] as String?,
      divisionName: json['division_name'] as String?,
      orderId: json['order_id'] as String?,
      orderNumber: json['order_number'] as String?,
      plannedReadyDate: json['planned_ready_date'] == null
          ? null
          : DateTime.parse(json['planned_ready_date'] as String),
      cancelReason: json['cancel_reason'] as String?,
      completedAt: json['completed_at'] == null
          ? null
          : DateTime.parse(json['completed_at'] as String),
      completedQuantity: json['completed_quantity'] as String?,
      orderLineId: json['order_line_id'] as String?,
      parentOrderId: json['parent_order_id'] as String?,
      startedAt: json['started_at'] == null
          ? null
          : DateTime.parse(json['started_at'] as String),
    );

Map<String, dynamic> _$ProductionOrderToJson(ProductionOrder instance) =>
    <String, dynamic>{
      'bom_revision': instance.bomRevision,
      'created_at': instance.createdAt.toIso8601String(),
      'customer_name': ?instance.customerName,
      'division_id': ?instance.divisionId,
      'division_name': ?instance.divisionName,
      'id': instance.id,
      'number': instance.number,
      'order_cancelled': instance.orderCancelled,
      'order_id': ?instance.orderId,
      'order_number': ?instance.orderNumber,
      'planned_ready_date': ?instance.plannedReadyDate?.toIso8601String(),
      'priority': instance.priority,
      'product_code': instance.productCode,
      'product_id': instance.productId,
      'product_name': instance.productName,
      'product_tracking': instance.productTracking,
      'quantity': instance.quantity,
      'short_components': instance.shortComponents,
      'status': instance.status,
      'unit': instance.unit,
      'cancel_reason': ?instance.cancelReason,
      'completed_at': ?instance.completedAt?.toIso8601String(),
      'completed_quantity': ?instance.completedQuantity,
      'components': instance.components,
      'notes': instance.notes,
      'order_line_id': ?instance.orderLineId,
      'outputs': instance.outputs,
      'parent_order_id': ?instance.parentOrderId,
      'started_at': ?instance.startedAt?.toIso8601String(),
    };
