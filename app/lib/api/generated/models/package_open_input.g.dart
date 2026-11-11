// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'package_open_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

PackageOpenInput _$PackageOpenInputFromJson(Map<String, dynamic> json) =>
    PackageOpenInput(
      kind: json['kind'] as String,
      customerId: json['customer_id'] as String?,
      note: json['note'] as String?,
    );

Map<String, dynamic> _$PackageOpenInputToJson(PackageOpenInput instance) =>
    <String, dynamic>{
      'customer_id': ?instance.customerId,
      'kind': instance.kind,
      'note': ?instance.note,
    };
