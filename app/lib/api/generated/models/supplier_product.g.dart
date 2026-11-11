// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'supplier_product.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

SupplierProduct _$SupplierProductFromJson(Map<String, dynamic> json) =>
    SupplierProduct(
      companyId: json['company_id'] as String,
      createdAt: DateTime.parse(json['created_at'] as String),
      currency: json['currency'] as String,
      id: json['id'] as String,
      isPreferred: json['is_preferred'] as bool,
      leadTimeDays: (json['lead_time_days'] as num).toInt(),
      moq: json['moq'] as String,
      productCode: json['product_code'] as String,
      productId: json['product_id'] as String,
      productName: json['product_name'] as String,
      supplierId: json['supplier_id'] as String,
      supplierName: json['supplier_name'] as String,
      supplierProductCode: json['supplier_product_code'] as String,
      supplierProductName: json['supplier_product_name'] as String,
      unitPrice: json['unit_price'] as String,
      updatedAt: DateTime.parse(json['updated_at'] as String),
    );

Map<String, dynamic> _$SupplierProductToJson(SupplierProduct instance) =>
    <String, dynamic>{
      'company_id': instance.companyId,
      'created_at': instance.createdAt.toIso8601String(),
      'currency': instance.currency,
      'id': instance.id,
      'is_preferred': instance.isPreferred,
      'lead_time_days': instance.leadTimeDays,
      'moq': instance.moq,
      'product_code': instance.productCode,
      'product_id': instance.productId,
      'product_name': instance.productName,
      'supplier_id': instance.supplierId,
      'supplier_name': instance.supplierName,
      'supplier_product_code': instance.supplierProductCode,
      'supplier_product_name': instance.supplierProductName,
      'unit_price': instance.unitPrice,
      'updated_at': instance.updatedAt.toIso8601String(),
    };
