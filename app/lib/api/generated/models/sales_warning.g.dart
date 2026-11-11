// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'sales_warning.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

SalesWarning _$SalesWarningFromJson(Map<String, dynamic> json) => SalesWarning(
  details: json['details'],
  kind: json['kind'] as String,
  message: json['message'] as String,
  productCode: json['product_code'] as String,
);

Map<String, dynamic> _$SalesWarningToJson(SalesWarning instance) =>
    <String, dynamic>{
      'details': instance.details,
      'kind': instance.kind,
      'message': instance.message,
      'product_code': instance.productCode,
    };
