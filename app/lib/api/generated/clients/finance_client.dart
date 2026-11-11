// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, unused_import, invalid_annotation_target, unnecessary_import

import 'package:dio/dio.dart';
import 'package:retrofit/retrofit.dart';

import '../models/approve_invoice_input.dart';
import '../models/dispute_invoice_input.dart';
import '../models/finance_settings.dart';
import '../models/invoice_detail.dart';
import '../models/invoice_page.dart';
import '../models/mark_paid_input.dart';
import '../models/new_invoice_input.dart';
import '../models/update_finance_settings_input.dart';
import '../models/update_invoice_input.dart';

part 'finance_client.g.dart';

@RestApi()
abstract class FinanceClient {
  factory FinanceClient(Dio dio, {String? baseUrl}) = _FinanceClient;

  @GET('/api/v1/finance/settings')
  Future<FinanceSettings> getSettings();

  @PUT('/api/v1/finance/settings')
  Future<FinanceSettings> updateSettings({
    @Body() required UpdateFinanceSettingsInput body,
  });

  /// [status] - draft | matched | disputed | approved | paid.
  ///
  /// [supplierId] - Only invoices of this supplier.
  ///
  /// [purchaseOrderId] - Only invoices against this purchase order.
  ///
  /// [q] - Searches the supplier's invoice number and the internal number.
  ///
  /// [limit] - Page size (default 50, at most 200).
  ///
  /// [offset] - Rows to skip.
  @GET('/api/v1/supplier-invoices')
  Future<InvoicePage> listInvoices({
    @Query('status') String? status,
    @Query('supplier_id') String? supplierId,
    @Query('purchase_order_id') String? purchaseOrderId,
    @Query('q') String? q,
    @Query('limit') int? limit,
    @Query('offset') int? offset,
  });

  @POST('/api/v1/supplier-invoices')
  Future<InvoiceDetail> createInvoice({
    @Body() required NewInvoiceInput body,
  });

  /// [id] - Supplier invoice ID
  @GET('/api/v1/supplier-invoices/{id}')
  Future<InvoiceDetail> getInvoice({
    @Path('id') required String id,
  });

  /// [id] - Supplier invoice ID
  @PUT('/api/v1/supplier-invoices/{id}')
  Future<InvoiceDetail> updateInvoice({
    @Path('id') required String id,
    @Body() required UpdateInvoiceInput body,
  });

  /// [id] - Supplier invoice ID
  @DELETE('/api/v1/supplier-invoices/{id}')
  Future<void> deleteInvoice({
    @Path('id') required String id,
  });

  /// [id] - Supplier invoice ID
  @POST('/api/v1/supplier-invoices/{id}/approve')
  Future<InvoiceDetail> approveInvoice({
    @Path('id') required String id,
    @Body() required ApproveInvoiceInput body,
  });

  /// [id] - Supplier invoice ID
  @POST('/api/v1/supplier-invoices/{id}/dispute')
  Future<InvoiceDetail> disputeInvoice({
    @Path('id') required String id,
    @Body() required DisputeInvoiceInput body,
  });

  /// [id] - Supplier invoice ID
  @POST('/api/v1/supplier-invoices/{id}/mark-paid')
  Future<InvoiceDetail> markPaid({
    @Path('id') required String id,
    @Body() required MarkPaidInput body,
  });

  /// [id] - Supplier invoice ID
  @POST('/api/v1/supplier-invoices/{id}/reopen')
  Future<InvoiceDetail> reopenInvoice({
    @Path('id') required String id,
  });

  /// [id] - Supplier invoice ID
  @POST('/api/v1/supplier-invoices/{id}/submit-match')
  Future<InvoiceDetail> submitMatch({
    @Path('id') required String id,
  });
}
