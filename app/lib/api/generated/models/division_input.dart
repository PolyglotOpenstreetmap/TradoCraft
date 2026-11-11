// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, unused_import, invalid_annotation_target, unnecessary_import

import 'package:json_annotation/json_annotation.dart';

part 'division_input.g.dart';

@JsonSerializable()
class DivisionInput {
  const DivisionInput({
    required this.code,
    required this.name,
    this.active,
    this.sortOrder,
  });
  
  factory DivisionInput.fromJson(Map<String, Object?> json) => _$DivisionInputFromJson(json);
  
  @JsonKey(includeIfNull: false)
  final bool? active;
  final String code;
  final String name;
  @JsonKey(includeIfNull: false,name: 'sort_order')
  final int? sortOrder;

  Map<String, Object?> toJson() => _$DivisionInputToJson(this);
}
