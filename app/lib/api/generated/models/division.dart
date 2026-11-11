// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, unused_import, invalid_annotation_target, unnecessary_import

import 'package:json_annotation/json_annotation.dart';

part 'division.g.dart';

@JsonSerializable()
class Division {
  const Division({
    required this.active,
    required this.code,
    required this.id,
    required this.name,
    required this.openOrders,
    required this.sortOrder,
  });
  
  factory Division.fromJson(Map<String, Object?> json) => _$DivisionFromJson(json);
  
  final bool active;
  final String code;
  final String id;
  final String name;

  /// Production orders in this division that are not finished.
  @JsonKey(name: 'open_orders')
  final int openOrders;
  @JsonKey(name: 'sort_order')
  final int sortOrder;

  Map<String, Object?> toJson() => _$DivisionToJson(this);
}
