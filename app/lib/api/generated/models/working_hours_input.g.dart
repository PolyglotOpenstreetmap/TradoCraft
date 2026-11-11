// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'working_hours_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

WorkingHoursInput _$WorkingHoursInputFromJson(Map<String, dynamic> json) =>
    WorkingHoursInput(
      slots: (json['slots'] as List<dynamic>)
          .map((e) => HoursSlotInput.fromJson(e as Map<String, dynamic>))
          .toList(),
      userId: json['user_id'] as String?,
    );

Map<String, dynamic> _$WorkingHoursInputToJson(WorkingHoursInput instance) =>
    <String, dynamic>{'slots': instance.slots, 'user_id': ?instance.userId};
