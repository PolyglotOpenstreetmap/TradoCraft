// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'settings_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

SettingsInput _$SettingsInputFromJson(Map<String, dynamic> json) =>
    SettingsInput(
      csatEnabled: json['csat_enabled'] as bool,
      defaultTeamId: json['default_team_id'] as String?,
    );

Map<String, dynamic> _$SettingsInputToJson(SettingsInput instance) =>
    <String, dynamic>{
      'csat_enabled': instance.csatEnabled,
      'default_team_id': ?instance.defaultTeamId,
    };
