use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use tradocraft_core::{AppResult, AppState, CurrentUser, Permission};
use utoipa::IntoParams;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use uuid::Uuid;

use crate::invoices::{
    self, ApproveInvoiceInput, DisputeInvoiceInput, InvoiceDetail, InvoiceFilter, InvoicePage,
    MarkPaidInput, NewInvoiceInput, UpdateInvoiceInput,
};
use crate::settings::{self, FinanceSettings, UpdateFinanceSettingsInput};

type ErrorBody = tradocraft_core::error::ErrorBody;

/// Query filters for listing supplier invoices.
#[derive(Debug, Deserialize, IntoParams)]
pub struct ListInvoicesParams {
    /// draft | matched | disputed | approved | paid
    pub status: Option<String>,
    /// Only invoices of this supplier.
    pub supplier_id: Option<Uuid>,
    /// Only invoices against this purchase order.
    pub purchase_order_id: Option<Uuid>,
    /// Searches the supplier's invoice number and the internal number.
    pub q: Option<String>,
    /// Page size (default 50, at most 200).
    pub limit: Option<i64>,
    /// Rows to skip.
    pub offset: Option<i64>,
}

#[utoipa::path(
    get,
    path = "/supplier-invoices",
    tag = "finance",
    params(ListInvoicesParams),
    responses(
        (status = 200, body = InvoicePage),
        (status = 401, body = ErrorBody),
        (status = 403, body = ErrorBody),
        (status = 422, body = ErrorBody)
    )
)]
async fn list_invoices(
    State(state): State<AppState>,
    user: CurrentUser,
    Query(p): Query<ListInvoicesParams>,
) -> AppResult<Json<InvoicePage>> {
    user.require(Permission::FinanceRead)?;
    let mut conn = state.db.acquire().await?;
    let page = invoices::list(
        &mut conn,
        user.company_id,
        InvoiceFilter {
            status: p.status,
            supplier_id: p.supplier_id,
            purchase_order_id: p.purchase_order_id,
            q: p.q,
            limit: p.limit,
            offset: p.offset,
        },
    )
    .await?;
    Ok(Json(page))
}

#[utoipa::path(
    post,
    path = "/supplier-invoices",
    tag = "finance",
    request_body = NewInvoiceInput,
    responses(
        (status = 201, body = InvoiceDetail),
        (status = 401, body = ErrorBody),
        (status = 403, body = ErrorBody),
        (status = 404, body = ErrorBody),
        (status = 409, body = ErrorBody),
        (status = 422, body = ErrorBody)
    )
)]
async fn create_invoice(
    State(state): State<AppState>,
    user: CurrentUser,
    Json(input): Json<NewInvoiceInput>,
) -> AppResult<(StatusCode, Json<InvoiceDetail>)> {
    user.require(Permission::FinanceWrite)?;
    let mut tx = state.db.begin().await?;
    let d = invoices::create(&mut tx, &user, input).await?;
    tx.commit().await?;
    Ok((StatusCode::CREATED, Json(d)))
}

#[utoipa::path(
    get,
    path = "/supplier-invoices/{id}",
    tag = "finance",
    params(("id" = Uuid, Path, description = "Supplier invoice ID")),
    responses(
        (status = 200, body = InvoiceDetail),
        (status = 401, body = ErrorBody),
        (status = 403, body = ErrorBody),
        (status = 404, body = ErrorBody)
    )
)]
async fn get_invoice(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<InvoiceDetail>> {
    user.require(Permission::FinanceRead)?;
    let mut conn = state.db.acquire().await?;
    Ok(Json(invoices::load(&mut conn, user.company_id, id).await?))
}

#[utoipa::path(
    put,
    path = "/supplier-invoices/{id}",
    tag = "finance",
    params(("id" = Uuid, Path, description = "Supplier invoice ID")),
    request_body = UpdateInvoiceInput,
    responses(
        (status = 200, body = InvoiceDetail),
        (status = 401, body = ErrorBody),
        (status = 403, body = ErrorBody),
        (status = 404, body = ErrorBody),
        (status = 409, body = ErrorBody),
        (status = 422, body = ErrorBody)
    )
)]
async fn update_invoice(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateInvoiceInput>,
) -> AppResult<Json<InvoiceDetail>> {
    user.require(Permission::FinanceWrite)?;
    let mut tx = state.db.begin().await?;
    let d = invoices::update(&mut tx, &user, id, input).await?;
    tx.commit().await?;
    Ok(Json(d))
}

#[utoipa::path(
    delete,
    path = "/supplier-invoices/{id}",
    tag = "finance",
    params(("id" = Uuid, Path, description = "Supplier invoice ID")),
    responses(
        (status = 204),
        (status = 401, body = ErrorBody),
        (status = 403, body = ErrorBody),
        (status = 404, body = ErrorBody),
        (status = 409, body = ErrorBody)
    )
)]
async fn delete_invoice(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    user.require(Permission::FinanceWrite)?;
    let mut tx = state.db.begin().await?;
    invoices::delete(&mut tx, &user, id).await?;
    tx.commit().await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    post,
    path = "/supplier-invoices/{id}/submit-match",
    tag = "finance",
    params(("id" = Uuid, Path, description = "Supplier invoice ID")),
    responses(
        (status = 200, body = InvoiceDetail),
        (status = 401, body = ErrorBody),
        (status = 403, body = ErrorBody),
        (status = 404, body = ErrorBody),
        (status = 409, body = ErrorBody)
    )
)]
async fn submit_match(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<InvoiceDetail>> {
    user.require(Permission::FinanceWrite)?;
    let mut tx = state.db.begin().await?;
    let d = invoices::submit_match(&mut tx, &user, id).await?;
    tx.commit().await?;
    Ok(Json(d))
}

#[utoipa::path(
    post,
    path = "/supplier-invoices/{id}/approve",
    tag = "finance",
    params(("id" = Uuid, Path, description = "Supplier invoice ID")),
    request_body = ApproveInvoiceInput,
    responses(
        (status = 200, body = InvoiceDetail),
        (status = 401, body = ErrorBody),
        (status = 403, body = ErrorBody),
        (status = 404, body = ErrorBody),
        (status = 409, body = ErrorBody)
    )
)]
async fn approve_invoice(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<Uuid>,
    Json(input): Json<ApproveInvoiceInput>,
) -> AppResult<Json<InvoiceDetail>> {
    user.require(Permission::FinanceApprove)?;
    let mut tx = state.db.begin().await?;
    let d = invoices::approve(&mut tx, &user, id, input).await?;
    tx.commit().await?;
    Ok(Json(d))
}

#[utoipa::path(
    post,
    path = "/supplier-invoices/{id}/dispute",
    tag = "finance",
    params(("id" = Uuid, Path, description = "Supplier invoice ID")),
    request_body = DisputeInvoiceInput,
    responses(
        (status = 200, body = InvoiceDetail),
        (status = 401, body = ErrorBody),
        (status = 403, body = ErrorBody),
        (status = 404, body = ErrorBody),
        (status = 409, body = ErrorBody),
        (status = 422, body = ErrorBody)
    )
)]
async fn dispute_invoice(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<Uuid>,
    Json(input): Json<DisputeInvoiceInput>,
) -> AppResult<Json<InvoiceDetail>> {
    user.require(Permission::FinanceWrite)?;
    let mut tx = state.db.begin().await?;
    let d = invoices::dispute(&mut tx, &user, id, input).await?;
    tx.commit().await?;
    Ok(Json(d))
}

#[utoipa::path(
    post,
    path = "/supplier-invoices/{id}/reopen",
    tag = "finance",
    params(("id" = Uuid, Path, description = "Supplier invoice ID")),
    responses(
        (status = 200, body = InvoiceDetail),
        (status = 401, body = ErrorBody),
        (status = 403, body = ErrorBody),
        (status = 404, body = ErrorBody),
        (status = 409, body = ErrorBody)
    )
)]
async fn reopen_invoice(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<InvoiceDetail>> {
    user.require(Permission::FinanceWrite)?;
    let mut tx = state.db.begin().await?;
    let d = invoices::reopen(&mut tx, &user, id).await?;
    tx.commit().await?;
    Ok(Json(d))
}

#[utoipa::path(
    post,
    path = "/supplier-invoices/{id}/mark-paid",
    tag = "finance",
    params(("id" = Uuid, Path, description = "Supplier invoice ID")),
    request_body = MarkPaidInput,
    responses(
        (status = 200, body = InvoiceDetail),
        (status = 401, body = ErrorBody),
        (status = 403, body = ErrorBody),
        (status = 404, body = ErrorBody),
        (status = 409, body = ErrorBody)
    )
)]
async fn mark_paid(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<Uuid>,
    Json(input): Json<MarkPaidInput>,
) -> AppResult<Json<InvoiceDetail>> {
    user.require(Permission::FinanceApprove)?;
    let mut tx = state.db.begin().await?;
    let d = invoices::mark_paid(&mut tx, &user, id, input).await?;
    tx.commit().await?;
    Ok(Json(d))
}

#[utoipa::path(
    get,
    path = "/finance/settings",
    tag = "finance",
    responses(
        (status = 200, body = FinanceSettings),
        (status = 401, body = ErrorBody),
        (status = 403, body = ErrorBody)
    )
)]
async fn get_settings(
    State(state): State<AppState>,
    user: CurrentUser,
) -> AppResult<Json<FinanceSettings>> {
    user.require(Permission::FinanceRead)?;
    let mut conn = state.db.acquire().await?;
    Ok(Json(settings::load(&mut conn, user.company_id).await?))
}

#[utoipa::path(
    put,
    path = "/finance/settings",
    tag = "finance",
    request_body = UpdateFinanceSettingsInput,
    responses(
        (status = 200, body = FinanceSettings),
        (status = 401, body = ErrorBody),
        (status = 403, body = ErrorBody),
        (status = 422, body = ErrorBody)
    )
)]
async fn update_settings(
    State(state): State<AppState>,
    user: CurrentUser,
    Json(input): Json<UpdateFinanceSettingsInput>,
) -> AppResult<Json<FinanceSettings>> {
    user.require(Permission::FinanceApprove)?;
    let mut tx = state.db.begin().await?;
    let s = settings::update(&mut tx, &user, input).await?;
    tx.commit().await?;
    Ok(Json(s))
}

/// Registers all finance routes with their OpenAPI declarations.
pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list_invoices, create_invoice))
        .routes(routes!(get_invoice, update_invoice, delete_invoice))
        .routes(routes!(submit_match))
        .routes(routes!(approve_invoice))
        .routes(routes!(dispute_invoice))
        .routes(routes!(reopen_invoice))
        .routes(routes!(mark_paid))
        .routes(routes!(get_settings, update_settings))
}
