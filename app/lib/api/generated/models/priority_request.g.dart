// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'priority_request.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

PriorityRequest _$PriorityRequestFromJson(Map<String, dynamic> json) =>
    PriorityRequest(
      createdAt: DateTime.parse(json['created_at'] as String),
      id: json['id'] as String,
      message: json['message'] as String,
      orderId: json['order_id'] as String,
      requestedByContactId: json['requested_by_contact_id'] as String,
      requestedByName: json['requested_by_name'] as String,
      requestedPriority: json['requested_priority'] as String,
      resolutionNote: json['resolution_note'] as String,
      status: json['status'] as String,
      resolvedAt: json['resolved_at'] == null
          ? null
          : DateTime.parse(json['resolved_at'] as String),
    );

Map<String, dynamic> _$PriorityRequestToJson(PriorityRequest instance) =>
    <String, dynamic>{
      'created_at': instance.createdAt.toIso8601String(),
      'id': instance.id,
      'message': instance.message,
      'order_id': instance.orderId,
      'requested_by_contact_id': instance.requestedByContactId,
      'requested_by_name': instance.requestedByName,
      'requested_priority': instance.requestedPriority,
      'resolution_note': instance.resolutionNote,
      'resolved_at': ?instance.resolvedAt?.toIso8601String(),
      'status': instance.status,
    };
