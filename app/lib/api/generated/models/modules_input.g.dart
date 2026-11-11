// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'modules_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ModulesInput _$ModulesInputFromJson(Map<String, dynamic> json) => ModulesInput(
  disabledModules: (json['disabled_modules'] as List<dynamic>)
      .map((e) => e as String)
      .toList(),
);

Map<String, dynamic> _$ModulesInputToJson(ModulesInput instance) =>
    <String, dynamic>{'disabled_modules': instance.disabledModules};
