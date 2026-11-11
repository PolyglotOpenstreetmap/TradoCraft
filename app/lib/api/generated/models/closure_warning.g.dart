// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'closure_warning.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ClosureWarning _$ClosureWarningFromJson(Map<String, dynamic> json) =>
    ClosureWarning(
      closure: Closure.fromJson(json['closure'] as Map<String, dynamic>),
      nextOpenDay: DateTime.parse(json['next_open_day'] as String),
    );

Map<String, dynamic> _$ClosureWarningToJson(ClosureWarning instance) =>
    <String, dynamic>{
      'closure': instance.closure,
      'next_open_day': instance.nextOpenDay.toIso8601String(),
    };
