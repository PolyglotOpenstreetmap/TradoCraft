// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'product_uoms_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ProductUomsInput _$ProductUomsInputFromJson(Map<String, dynamic> json) =>
    ProductUomsInput(
      uoms: (json['uoms'] as List<dynamic>)
          .map((e) => ProductUom.fromJson(e as Map<String, dynamic>))
          .toList(),
    );

Map<String, dynamic> _$ProductUomsInputToJson(ProductUomsInput instance) =>
    <String, dynamic>{'uoms': instance.uoms};
