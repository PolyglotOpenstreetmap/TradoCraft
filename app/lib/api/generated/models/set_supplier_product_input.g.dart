// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'set_supplier_product_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

SetSupplierProductInput _$SetSupplierProductInputFromJson(
  Map<String, dynamic> json,
) => SetSupplierProductInput(
  unitPrice: json['unit_price'] as String,
  currency: json['currency'] as String?,
  isPreferred: json['is_preferred'] as bool?,
  leadTimeDays: (json['lead_time_days'] as num?)?.toInt(),
  moq: json['moq'] as String?,
  supplierProductCode: json['supplier_product_code'] as String?,
  supplierProductName: json['supplier_product_name'] as String?,
);

Map<String, dynamic> _$SetSupplierProductInputToJson(
  SetSupplierProductInput instance,
) => <String, dynamic>{
  'currency': ?instance.currency,
  'is_preferred': ?instance.isPreferred,
  'lead_time_days': ?instance.leadTimeDays,
  'moq': ?instance.moq,
  'supplier_product_code': ?instance.supplierProductCode,
  'supplier_product_name': ?instance.supplierProductName,
  'unit_price': instance.unitPrice,
};
