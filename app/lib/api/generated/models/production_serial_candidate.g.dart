// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'production_serial_candidate.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ProductionSerialCandidate _$ProductionSerialCandidateFromJson(
  Map<String, dynamic> json,
) => ProductionSerialCandidate(
  id: json['id'] as String,
  location: json['location'] as String,
  locationId: json['location_id'] as String,
  serialNumber: json['serial_number'] as String,
);

Map<String, dynamic> _$ProductionSerialCandidateToJson(
  ProductionSerialCandidate instance,
) => <String, dynamic>{
  'id': instance.id,
  'location': instance.location,
  'location_id': instance.locationId,
  'serial_number': instance.serialNumber,
};
