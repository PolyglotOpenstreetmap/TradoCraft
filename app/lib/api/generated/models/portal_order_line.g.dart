// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'portal_order_line.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

PortalOrderLine _$PortalOrderLineFromJson(Map<String, dynamic> json) =>
    PortalOrderLine(
      description: json['description'] as String,
      lineTotal: json['line_total'] as String,
      listUnitPrice: json['list_unit_price'] as String,
      netUnitPrice: json['net_unit_price'] as String,
      position: (json['position'] as num).toInt(),
      productCode: json['product_code'] as String,
      quantity: json['quantity'] as String,
      unit: json['unit'] as String,
      vatRate: json['vat_rate'] as String,
    );

Map<String, dynamic> _$PortalOrderLineToJson(PortalOrderLine instance) =>
    <String, dynamic>{
      'description': instance.description,
      'line_total': instance.lineTotal,
      'list_unit_price': instance.listUnitPrice,
      'net_unit_price': instance.netUnitPrice,
      'position': instance.position,
      'product_code': instance.productCode,
      'quantity': instance.quantity,
      'unit': instance.unit,
      'vat_rate': instance.vatRate,
    };
