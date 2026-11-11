// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'helpdesk_settings.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

HelpdeskSettings _$HelpdeskSettingsFromJson(Map<String, dynamic> json) =>
    HelpdeskSettings(
      csatEnabled: json['csat_enabled'] as bool,
      defaultTeamId: json['default_team_id'] as String?,
    );

Map<String, dynamic> _$HelpdeskSettingsToJson(HelpdeskSettings instance) =>
    <String, dynamic>{
      'csat_enabled': instance.csatEnabled,
      'default_team_id': ?instance.defaultTeamId,
    };
