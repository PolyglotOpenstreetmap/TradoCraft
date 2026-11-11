// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'opportunity_page.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

OpportunityPage _$OpportunityPageFromJson(Map<String, dynamic> json) =>
    OpportunityPage(
      items: (json['items'] as List<dynamic>)
          .map((e) => Opportunity.fromJson(e as Map<String, dynamic>))
          .toList(),
      total: (json['total'] as num).toInt(),
    );

Map<String, dynamic> _$OpportunityPageToJson(OpportunityPage instance) =>
    <String, dynamic>{'items': instance.items, 'total': instance.total};
