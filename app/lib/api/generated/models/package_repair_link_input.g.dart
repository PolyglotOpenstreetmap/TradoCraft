// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'package_repair_link_input.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

PackageRepairLinkInput _$PackageRepairLinkInputFromJson(
  Map<String, dynamic> json,
) => PackageRepairLinkInput(
  clientId: json['client_id'] as String?,
  confirmCustomerMismatch: json['confirm_customer_mismatch'] as bool?,
  expectedStatus: json['expected_status'] as String?,
  newRepair: json['new_repair'] == null
      ? null
      : PackageNewRepairInput.fromJson(
          json['new_repair'] as Map<String, dynamic>,
        ),
  repairId: json['repair_id'] as String?,
);

Map<String, dynamic> _$PackageRepairLinkInputToJson(
  PackageRepairLinkInput instance,
) => <String, dynamic>{
  'client_id': ?instance.clientId,
  'confirm_customer_mismatch': ?instance.confirmCustomerMismatch,
  'expected_status': ?instance.expectedStatus,
  'new_repair': ?instance.newRepair,
  'repair_id': ?instance.repairId,
};
