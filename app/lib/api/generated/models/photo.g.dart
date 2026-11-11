// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'photo.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Photo _$PhotoFromJson(Map<String, dynamic> json) => Photo(
  caption: json['caption'] as String,
  contentType: json['content_type'] as String,
  createdAt: DateTime.parse(json['created_at'] as String),
  filename: json['filename'] as String,
  id: json['id'] as String,
  sizeBytes: (json['size_bytes'] as num).toInt(),
  uploaderName: json['uploader_name'] as String,
);

Map<String, dynamic> _$PhotoToJson(Photo instance) => <String, dynamic>{
  'caption': instance.caption,
  'content_type': instance.contentType,
  'created_at': instance.createdAt.toIso8601String(),
  'filename': instance.filename,
  'id': instance.id,
  'size_bytes': instance.sizeBytes,
  'uploader_name': instance.uploaderName,
};
