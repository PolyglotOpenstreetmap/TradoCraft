// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'receive_goods_line_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ReceiveGoodsLineInput _$ReceiveGoodsLineInputFromJson(
  Map<String, dynamic> json,
) => ReceiveGoodsLineInput(
  poLineId: json['po_line_id'] as String,
  quantity: json['quantity'] as String,
  lotNumber: json['lot_number'] as String?,
  serialNumbers: (json['serial_numbers'] as List<dynamic>?)
      ?.map((e) => e as String)
      .toList(),
);

Map<String, dynamic> _$ReceiveGoodsLineInputToJson(
  ReceiveGoodsLineInput instance,
) => <String, dynamic>{
  'lot_number': ?instance.lotNumber,
  'po_line_id': instance.poLineId,
  'quantity': instance.quantity,
  'serial_numbers': ?instance.serialNumbers,
};
