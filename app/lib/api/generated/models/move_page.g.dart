// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'move_page.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

MovePage _$MovePageFromJson(Map<String, dynamic> json) => MovePage(
  items: (json['items'] as List<dynamic>)
      .map((e) => StockMove.fromJson(e as Map<String, dynamic>))
      .toList(),
  total: (json['total'] as num).toInt(),
);

Map<String, dynamic> _$MovePageToJson(MovePage instance) => <String, dynamic>{
  'items': instance.items,
  'total': instance.total,
};
