// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'expr_error.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ExprError _$ExprErrorFromJson(Map<String, dynamic> json) => ExprError(
  end: (json['end'] as num).toInt(),
  message: json['message'] as String,
  start: (json['start'] as num).toInt(),
);

Map<String, dynamic> _$ExprErrorToJson(ExprError instance) => <String, dynamic>{
  'end': instance.end,
  'message': instance.message,
  'start': instance.start,
};
