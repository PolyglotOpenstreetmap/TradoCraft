// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, unused_import, invalid_annotation_target, unnecessary_import

import 'package:dio/dio.dart';
import 'package:retrofit/retrofit.dart';

import '../models/goods_receipt_detail.dart';
import '../models/goods_receipt_summary.dart';
import '../models/new_purchase_order_input.dart';
import '../models/new_supplier_input.dart';
import '../models/purchase_order_detail.dart';
import '../models/purchase_order_summary.dart';
import '../models/receive_goods_input.dart';
import '../models/set_supplier_product_input.dart';
import '../models/supplier.dart';
import '../models/supplier_product.dart';
import '../models/update_supplier_input.dart';

part 'purchasing_client.g.dart';

@RestApi()
abstract class PurchasingClient {
  factory PurchasingClient(Dio dio, {String? baseUrl}) = _PurchasingClient;

  /// [id] - Goods Receipt ID
  @GET('/api/v1/goods-receipts/{id}')
  Future<GoodsReceiptDetail> getReceipt({
    @Path('id') required String id,
  });

  /// [id] - Product ID
  @GET('/api/v1/products/{id}/suppliers')
  Future<List<SupplierProduct>> listProductSuppliers({
    @Path('id') required String id,
  });

  @GET('/api/v1/purchase-orders')
  Future<List<PurchaseOrderSummary>> listOrders({
    @Query('supplier_id') String? supplierId,
    @Query('status') String? status,
  });

  @POST('/api/v1/purchase-orders')
  Future<PurchaseOrderDetail> createOrder({
    @Body() required NewPurchaseOrderInput body,
  });

  /// [id] - Purchase Order ID
  @GET('/api/v1/purchase-orders/{id}')
  Future<PurchaseOrderDetail> getOrder({
    @Path('id') required String id,
  });

  /// [id] - Purchase Order ID
  @POST('/api/v1/purchase-orders/{id}/approve')
  Future<PurchaseOrderDetail> approveOrder({
    @Path('id') required String id,
  });

  /// [id] - Purchase Order ID
  @POST('/api/v1/purchase-orders/{id}/cancel')
  Future<PurchaseOrderDetail> cancelOrder({
    @Path('id') required String id,
  });

  /// [id] - Purchase Order ID
  @GET('/api/v1/purchase-orders/{id}/receipts')
  Future<List<GoodsReceiptSummary>> listOrderReceipts({
    @Path('id') required String id,
  });

  /// [id] - Purchase Order ID
  @POST('/api/v1/purchase-orders/{id}/receipts')
  Future<GoodsReceiptDetail> receiveGoods({
    @Path('id') required String id,
    @Body() required ReceiveGoodsInput body,
  });

  /// [id] - Purchase Order ID
  @POST('/api/v1/purchase-orders/{id}/send')
  Future<PurchaseOrderDetail> sendOrder({
    @Path('id') required String id,
  });

  @GET('/api/v1/suppliers')
  Future<List<Supplier>> listSuppliers({
    @Query('active_only') bool? activeOnly,
  });

  @POST('/api/v1/suppliers')
  Future<Supplier> createSupplier({
    @Body() required NewSupplierInput body,
  });

  /// [id] - Supplier ID
  @GET('/api/v1/suppliers/{id}')
  Future<Supplier> getSupplier({
    @Path('id') required String id,
  });

  /// [id] - Supplier ID
  @PUT('/api/v1/suppliers/{id}')
  Future<Supplier> updateSupplier({
    @Path('id') required String id,
    @Body() required UpdateSupplierInput body,
  });

  /// [id] - Supplier ID
  @GET('/api/v1/suppliers/{id}/products')
  Future<List<SupplierProduct>> listSupplierProducts({
    @Path('id') required String id,
  });

  /// [id] - Supplier ID.
  ///
  /// [productId] - Product ID.
  @POST('/api/v1/suppliers/{id}/products/{product_id}')
  Future<SupplierProduct> setSupplierProduct({
    @Path('id') required String id,
    @Path('product_id') required String productId,
    @Body() required SetSupplierProductInput body,
  });

  /// [id] - Supplier ID.
  ///
  /// [productId] - Product ID.
  @DELETE('/api/v1/suppliers/{id}/products/{product_id}')
  Future<void> deleteSupplierProduct({
    @Path('id') required String id,
    @Path('product_id') required String productId,
  });
}
