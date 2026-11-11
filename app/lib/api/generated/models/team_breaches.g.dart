// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'team_breaches.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

TeamBreaches _$TeamBreachesFromJson(Map<String, dynamic> json) => TeamBreaches(
  breached: (json['breached'] as num).toInt(),
  team: json['team'] as String,
  tickets: (json['tickets'] as num).toInt(),
  teamId: json['team_id'] as String?,
);

Map<String, dynamic> _$TeamBreachesToJson(TeamBreaches instance) =>
    <String, dynamic>{
      'breached': instance.breached,
      'team': instance.team,
      'team_id': ?instance.teamId,
      'tickets': instance.tickets,
    };
