// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'portal_order_detail.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

PortalOrderDetail _$PortalOrderDetailFromJson(Map<String, dynamic> json) =>
    PortalOrderDetail(
      lines: (json['lines'] as List<dynamic>)
          .map((e) => PortalOrderLine.fromJson(e as Map<String, dynamic>))
          .toList(),
      order: PortalOrder.fromJson(json['order'] as Map<String, dynamic>),
    );

Map<String, dynamic> _$PortalOrderDetailToJson(PortalOrderDetail instance) =>
    <String, dynamic>{'lines': instance.lines, 'order': instance.order};
