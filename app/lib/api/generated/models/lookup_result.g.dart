// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'lookup_result.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

LookupResult _$LookupResultFromJson(Map<String, dynamic> json) => LookupResult(
  contacts: (json['contacts'] as List<dynamic>)
      .map((e) => Contact.fromJson(e as Map<String, dynamic>))
      .toList(),
  customers: (json['customers'] as List<dynamic>)
      .map((e) => CustomerMatch.fromJson(e as Map<String, dynamic>))
      .toList(),
);

Map<String, dynamic> _$LookupResultToJson(LookupResult instance) =>
    <String, dynamic>{
      'contacts': instance.contacts,
      'customers': instance.customers,
    };
