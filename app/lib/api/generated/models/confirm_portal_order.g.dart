// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'confirm_portal_order.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ConfirmPortalOrder _$ConfirmPortalOrderFromJson(Map<String, dynamic> json) =>
    ConfirmPortalOrder(
      revision: (json['revision'] as num).toInt(),
      note: json['note'] as String?,
    );

Map<String, dynamic> _$ConfirmPortalOrderToJson(ConfirmPortalOrder instance) =>
    <String, dynamic>{'note': ?instance.note, 'revision': instance.revision};
