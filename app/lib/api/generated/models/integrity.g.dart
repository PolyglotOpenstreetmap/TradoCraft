// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'integrity.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Integrity _$IntegrityFromJson(Map<String, dynamic> json) => Integrity(
  differences: (json['differences'] as List<dynamic>)
      .map((e) => BalanceDifference.fromJson(e as Map<String, dynamic>))
      .toList(),
  ok: json['ok'] as bool,
);

Map<String, dynamic> _$IntegrityToJson(Integrity instance) => <String, dynamic>{
  'differences': instance.differences,
  'ok': instance.ok,
};
