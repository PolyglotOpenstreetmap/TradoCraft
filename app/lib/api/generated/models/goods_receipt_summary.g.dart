// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'goods_receipt_summary.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

GoodsReceiptSummary _$GoodsReceiptSummaryFromJson(Map<String, dynamic> json) =>
    GoodsReceiptSummary(
      carrier: json['carrier'] as String,
      companyId: json['company_id'] as String,
      createdAt: DateTime.parse(json['created_at'] as String),
      id: json['id'] as String,
      locationId: json['location_id'] as String,
      notes: json['notes'] as String,
      number: json['number'] as String,
      purchaseOrderId: json['purchase_order_id'] as String,
      purchaseOrderNumber: json['purchase_order_number'] as String,
      receivedByUserId: json['received_by_user_id'] as String,
      receivedDate: DateTime.parse(json['received_date'] as String),
      supplierDeliveryNote: json['supplier_delivery_note'] as String,
      supplierName: json['supplier_name'] as String,
      trackingNumber: json['tracking_number'] as String,
      warehouseId: json['warehouse_id'] as String,
    );

Map<String, dynamic> _$GoodsReceiptSummaryToJson(
  GoodsReceiptSummary instance,
) => <String, dynamic>{
  'carrier': instance.carrier,
  'company_id': instance.companyId,
  'created_at': instance.createdAt.toIso8601String(),
  'id': instance.id,
  'location_id': instance.locationId,
  'notes': instance.notes,
  'number': instance.number,
  'purchase_order_id': instance.purchaseOrderId,
  'purchase_order_number': instance.purchaseOrderNumber,
  'received_by_user_id': instance.receivedByUserId,
  'received_date': instance.receivedDate.toIso8601String(),
  'supplier_delivery_note': instance.supplierDeliveryNote,
  'supplier_name': instance.supplierName,
  'tracking_number': instance.trackingNumber,
  'warehouse_id': instance.warehouseId,
};
