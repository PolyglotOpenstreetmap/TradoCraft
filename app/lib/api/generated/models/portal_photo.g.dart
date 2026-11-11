// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'portal_photo.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

PortalPhoto _$PortalPhotoFromJson(Map<String, dynamic> json) => PortalPhoto(
  caption: json['caption'] as String,
  createdAt: DateTime.parse(json['created_at'] as String),
  filename: json['filename'] as String,
  id: json['id'] as String,
  uploaderName: json['uploader_name'] as String,
);

Map<String, dynamic> _$PortalPhotoToJson(PortalPhoto instance) =>
    <String, dynamic>{
      'caption': instance.caption,
      'created_at': instance.createdAt.toIso8601String(),
      'filename': instance.filename,
      'id': instance.id,
      'uploader_name': instance.uploaderName,
    };
