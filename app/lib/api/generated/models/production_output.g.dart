// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'production_output.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ProductionOutput _$ProductionOutputFromJson(Map<String, dynamic> json) =>
    ProductionOutput(
      id: json['id'] as String,
      quantity: json['quantity'] as String,
      stockMoveId: json['stock_move_id'] as String,
      lotNumber: json['lot_number'] as String?,
      serialNumber: json['serial_number'] as String?,
    );

Map<String, dynamic> _$ProductionOutputToJson(ProductionOutput instance) =>
    <String, dynamic>{
      'id': instance.id,
      'lot_number': ?instance.lotNumber,
      'quantity': instance.quantity,
      'serial_number': ?instance.serialNumber,
      'stock_move_id': instance.stockMoveId,
    };
