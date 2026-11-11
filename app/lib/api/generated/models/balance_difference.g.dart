// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'balance_difference.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

BalanceDifference _$BalanceDifferenceFromJson(Map<String, dynamic> json) =>
    BalanceDifference(
      balanceQuantity: json['balance_quantity'] as String,
      ledgerQuantity: json['ledger_quantity'] as String,
      locationId: json['location_id'] as String,
      productId: json['product_id'] as String,
      lotId: json['lot_id'] as String?,
    );

Map<String, dynamic> _$BalanceDifferenceToJson(BalanceDifference instance) =>
    <String, dynamic>{
      'balance_quantity': instance.balanceQuantity,
      'ledger_quantity': instance.ledgerQuantity,
      'location_id': instance.locationId,
      'lot_id': ?instance.lotId,
      'product_id': instance.productId,
    };
