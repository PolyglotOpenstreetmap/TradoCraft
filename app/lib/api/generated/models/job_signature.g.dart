// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'job_signature.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

JobSignature _$JobSignatureFromJson(Map<String, dynamic> json) => JobSignature(
  declineReason: json['decline_reason'] as String,
  declined: json['declined'] as bool,
  hasImage: json['has_image'] as bool,
  signedAt: DateTime.parse(json['signed_at'] as String),
  signerName: json['signer_name'] as String,
  capturedByName: json['captured_by_name'] as String?,
);

Map<String, dynamic> _$JobSignatureToJson(JobSignature instance) =>
    <String, dynamic>{
      'captured_by_name': ?instance.capturedByName,
      'decline_reason': instance.declineReason,
      'declined': instance.declined,
      'has_image': instance.hasImage,
      'signed_at': instance.signedAt.toIso8601String(),
      'signer_name': instance.signerName,
    };
