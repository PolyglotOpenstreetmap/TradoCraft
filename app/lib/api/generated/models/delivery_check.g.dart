// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'delivery_check.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

DeliveryCheck _$DeliveryCheckFromJson(Map<String, dynamic> json) =>
    DeliveryCheck(
      closed: json['closed'] as bool,
      date: DateTime.parse(json['date'] as String),
      nextOpenDay: DateTime.parse(json['next_open_day'] as String),
      closure: json['closure'] == null
          ? null
          : Closure.fromJson(json['closure'] as Map<String, dynamic>),
    );

Map<String, dynamic> _$DeliveryCheckToJson(DeliveryCheck instance) =>
    <String, dynamic>{
      'closed': instance.closed,
      'closure': ?instance.closure,
      'date': instance.date.toIso8601String(),
      'next_open_day': instance.nextOpenDay.toIso8601String(),
    };
