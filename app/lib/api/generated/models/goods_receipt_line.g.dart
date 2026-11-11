// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'goods_receipt_line.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

GoodsReceiptLine _$GoodsReceiptLineFromJson(Map<String, dynamic> json) =>
    GoodsReceiptLine(
      createdAt: DateTime.parse(json['created_at'] as String),
      goodsReceiptId: json['goods_receipt_id'] as String,
      id: json['id'] as String,
      poLineId: json['po_line_id'] as String,
      productCode: json['product_code'] as String,
      productId: json['product_id'] as String,
      productName: json['product_name'] as String,
      quantityReceived: json['quantity_received'] as String,
      lotNumber: json['lot_number'] as String?,
    );

Map<String, dynamic> _$GoodsReceiptLineToJson(GoodsReceiptLine instance) =>
    <String, dynamic>{
      'created_at': instance.createdAt.toIso8601String(),
      'goods_receipt_id': instance.goodsReceiptId,
      'id': instance.id,
      'lot_number': ?instance.lotNumber,
      'po_line_id': instance.poLineId,
      'product_code': instance.productCode,
      'product_id': instance.productId,
      'product_name': instance.productName,
      'quantity_received': instance.quantityReceived,
    };
