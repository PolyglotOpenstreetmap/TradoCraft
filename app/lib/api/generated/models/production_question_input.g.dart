// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'production_question_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ProductionQuestionInput _$ProductionQuestionInputFromJson(
  Map<String, dynamic> json,
) => ProductionQuestionInput(
  body: json['body'] as String,
  orderId: json['order_id'] as String,
  subject: json['subject'] as String,
);

Map<String, dynamic> _$ProductionQuestionInputToJson(
  ProductionQuestionInput instance,
) => <String, dynamic>{
  'body': instance.body,
  'order_id': instance.orderId,
  'subject': instance.subject,
};
