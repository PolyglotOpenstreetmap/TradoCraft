// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'row_error.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

RowError _$RowErrorFromJson(Map<String, dynamic> json) => RowError(
  message: json['message'] as String,
  row: (json['row'] as num).toInt(),
  column: json['column'] as String?,
);

Map<String, dynamic> _$RowErrorToJson(RowError instance) => <String, dynamic>{
  'column': ?instance.column,
  'message': instance.message,
  'row': instance.row,
};
