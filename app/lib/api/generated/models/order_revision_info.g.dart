// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'order_revision_info.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

OrderRevisionInfo _$OrderRevisionInfoFromJson(Map<String, dynamic> json) =>
    OrderRevisionInfo(
      createdAt: DateTime.parse(json['created_at'] as String),
      revision: (json['revision'] as num).toInt(),
      status: json['status'] as String,
      subtotal: json['subtotal'] as String,
      total: json['total'] as String,
      sentAt: json['sent_at'] == null
          ? null
          : DateTime.parse(json['sent_at'] as String),
      sentTo: json['sent_to'] as String?,
    );

Map<String, dynamic> _$OrderRevisionInfoToJson(OrderRevisionInfo instance) =>
    <String, dynamic>{
      'created_at': instance.createdAt.toIso8601String(),
      'revision': instance.revision,
      'sent_at': ?instance.sentAt?.toIso8601String(),
      'sent_to': ?instance.sentTo,
      'status': instance.status,
      'subtotal': instance.subtotal,
      'total': instance.total,
    };
