// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'package_repair_suggestions.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

PackageRepairSuggestions _$PackageRepairSuggestionsFromJson(
  Map<String, dynamic> json,
) => PackageRepairSuggestions(
  suggestions: (json['suggestions'] as List<dynamic>)
      .map((e) => RepairSuggestion.fromJson(e as Map<String, dynamic>))
      .toList(),
  proposedCustomerId: json['proposed_customer_id'] as String?,
  proposedCustomerName: json['proposed_customer_name'] as String?,
);

Map<String, dynamic> _$PackageRepairSuggestionsToJson(
  PackageRepairSuggestions instance,
) => <String, dynamic>{
  'proposed_customer_id': ?instance.proposedCustomerId,
  'proposed_customer_name': ?instance.proposedCustomerName,
  'suggestions': instance.suggestions,
};
