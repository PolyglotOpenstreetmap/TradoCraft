// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'closure.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Closure _$ClosureFromJson(Map<String, dynamic> json) => Closure(
  createdAt: DateTime.parse(json['created_at'] as String),
  customerCode: json['customer_code'] as String,
  customerId: json['customer_id'] as String,
  customerName: json['customer_name'] as String,
  endsOn: DateTime.parse(json['ends_on'] as String),
  id: json['id'] as String,
  kind: json['kind'] as String,
  notes: json['notes'] as String,
  source: json['source'] as String,
  startsOn: DateTime.parse(json['starts_on'] as String),
);

Map<String, dynamic> _$ClosureToJson(Closure instance) => <String, dynamic>{
  'created_at': instance.createdAt.toIso8601String(),
  'customer_code': instance.customerCode,
  'customer_id': instance.customerId,
  'customer_name': instance.customerName,
  'ends_on': instance.endsOn.toIso8601String(),
  'id': instance.id,
  'kind': instance.kind,
  'notes': instance.notes,
  'source': instance.source,
  'starts_on': instance.startsOn.toIso8601String(),
};
