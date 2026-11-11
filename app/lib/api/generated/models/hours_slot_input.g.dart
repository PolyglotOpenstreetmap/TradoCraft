// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'hours_slot_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

HoursSlotInput _$HoursSlotInputFromJson(Map<String, dynamic> json) =>
    HoursSlotInput(
      endTime: json['end_time'] as String,
      startTime: json['start_time'] as String,
      weekday: (json['weekday'] as num).toInt(),
    );

Map<String, dynamic> _$HoursSlotInputToJson(HoursSlotInput instance) =>
    <String, dynamic>{
      'end_time': instance.endTime,
      'start_time': instance.startTime,
      'weekday': instance.weekday,
    };
