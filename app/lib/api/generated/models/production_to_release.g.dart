// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'production_to_release.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ProductionToRelease _$ProductionToReleaseFromJson(Map<String, dynamic> json) =>
    ProductionToRelease(
      customerName: json['customer_name'] as String,
      orderId: json['order_id'] as String,
      orderLineId: json['order_line_id'] as String,
      orderNumber: json['order_number'] as String,
      ordered: json['ordered'] as String,
      planned: json['planned'] as String,
      priority: json['priority'] as String,
      productCode: json['product_code'] as String,
      productId: json['product_id'] as String,
      productName: json['product_name'] as String,
      toBuild: json['to_build'] as String,
    );

Map<String, dynamic> _$ProductionToReleaseToJson(
  ProductionToRelease instance,
) => <String, dynamic>{
  'customer_name': instance.customerName,
  'order_id': instance.orderId,
  'order_line_id': instance.orderLineId,
  'order_number': instance.orderNumber,
  'ordered': instance.ordered,
  'planned': instance.planned,
  'priority': instance.priority,
  'product_code': instance.productCode,
  'product_id': instance.productId,
  'product_name': instance.productName,
  'to_build': instance.toBuild,
};
