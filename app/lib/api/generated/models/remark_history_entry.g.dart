// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'remark_history_entry.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

RemarkHistoryEntry _$RemarkHistoryEntryFromJson(Map<String, dynamic> json) =>
    RemarkHistoryEntry(
      lastQuotationNumber: json['last_quotation_number'] as String,
      lastUsed: DateTime.parse(json['last_used'] as String),
      text: json['text'] as String,
      useCount: (json['use_count'] as num).toInt(),
    );

Map<String, dynamic> _$RemarkHistoryEntryToJson(RemarkHistoryEntry instance) =>
    <String, dynamic>{
      'last_quotation_number': instance.lastQuotationNumber,
      'last_used': instance.lastUsed.toIso8601String(),
      'text': instance.text,
      'use_count': instance.useCount,
    };
