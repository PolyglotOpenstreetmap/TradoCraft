// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'vat_group.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

VatGroup _$VatGroupFromJson(Map<String, dynamic> json) => VatGroup(
  baseValue: json['base'] as String,
  vat: json['vat'] as String,
  vatRate: json['vat_rate'] as String,
);

Map<String, dynamic> _$VatGroupToJson(VatGroup instance) => <String, dynamic>{
  'base': instance.baseValue,
  'vat': instance.vat,
  'vat_rate': instance.vatRate,
};
