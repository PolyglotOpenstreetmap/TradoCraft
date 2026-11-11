//! Production divisions: the teams orders are queued for, and which division normally builds a product.

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tradocraft_core::{audit, new_id, AppError, AppResult, AppState, CurrentUser, Permission};
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use uuid::Uuid;

type ErrorBody = tradocraft_core::error::ErrorBody;

#[derive(Debug, Clone, Serialize, FromRow, ToSchema)]
pub struct Division {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub sort_order: i32,
    pub active: bool,
    /// Production orders in this division that are not finished.
    pub open_orders: i64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct DivisionInput {
    pub code: String,
    pub name: String,
    #[serde(default)]
    pub sort_order: i32,
    #[serde(default = "default_true")]
    pub active: bool,
}

/// Serde default: a new division is active.
fn default_true() -> bool {
    true
}

impl DivisionInput {
    /// Code and name are required.
    fn check(&self) -> AppResult<()> {
        if self.code.trim().is_empty() || self.name.trim().is_empty() {
            return Err(AppError::Validation("code and name are required".into()));
        }
        Ok(())
    }
}

const SELECT: &str = "SELECT d.id, d.code, d.name, d.sort_order, d.active,
        (SELECT count(*) FROM production.production_orders po
         WHERE po.division_id = d.id AND po.status IN ('released', 'in_progress')) AS open_orders
     FROM production.divisions d";

/// The divisions of the company, in their sort order.
#[utoipa::path(get, path = "/production/divisions", tag = "production",
    responses((status = 200, body = Vec<Division>)))]
async fn list_divisions(
    State(state): State<AppState>,
    user: CurrentUser,
) -> AppResult<Json<Vec<Division>>> {
    user.require(Permission::ProductionRead)?;
    Ok(Json(
        sqlx::query_as(&format!(
            "{SELECT} WHERE d.company_id = $1 ORDER BY d.sort_order, d.code"
        ))
        .bind(user.company_id)
        .fetch_all(&state.db)
        .await?,
    ))
}

/// Adds a division.
#[utoipa::path(post, path = "/production/divisions", tag = "production", request_body = DivisionInput,
    responses((status = 201, body = Division), (status = 409, body = ErrorBody), (status = 422, body = ErrorBody)))]
async fn create_division(
    State(state): State<AppState>,
    user: CurrentUser,
    Json(input): Json<DivisionInput>,
) -> AppResult<(StatusCode, Json<Division>)> {
    user.require(Permission::ProductionAdmin)?;
    input.check()?;
    let mut tx = state.db.begin().await?;
    let id = new_id();
    sqlx::query(
        "INSERT INTO production.divisions (id, company_id, code, name, sort_order, active)
         VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(id)
    .bind(user.company_id)
    .bind(input.code.trim())
    .bind(input.name.trim())
    .bind(input.sort_order)
    .bind(input.active)
    .execute(&mut *tx)
    .await?;
    audit::record(
        &mut tx,
        &user,
        "production_division",
        id,
        "created",
        serde_json::json!({ "code": input.code.trim(), "name": input.name.trim() }),
    )
    .await?;
    let d = sqlx::query_as(&format!("{SELECT} WHERE d.id = $1"))
        .bind(id)
        .fetch_one(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok((StatusCode::CREATED, Json(d)))
}

/// Changes a division.
#[utoipa::path(put, path = "/production/divisions/{id}", tag = "production", params(("id" = Uuid, Path)),
    request_body = DivisionInput,
    responses((status = 200, body = Division), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody)))]
async fn update_division(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<Uuid>,
    Json(input): Json<DivisionInput>,
) -> AppResult<Json<Division>> {
    user.require(Permission::ProductionAdmin)?;
    input.check()?;
    let mut tx = state.db.begin().await?;
    let done = sqlx::query(
        "UPDATE production.divisions SET code = $3, name = $4, sort_order = $5, active = $6
         WHERE id = $1 AND company_id = $2",
    )
    .bind(id)
    .bind(user.company_id)
    .bind(input.code.trim())
    .bind(input.name.trim())
    .bind(input.sort_order)
    .bind(input.active)
    .execute(&mut *tx)
    .await?;
    if done.rows_affected() == 0 {
        return Err(AppError::NotFound("division"));
    }
    audit::record(
        &mut tx,
        &user,
        "production_division",
        id,
        "updated",
        serde_json::json!({ "code": input.code.trim(), "active": input.active }),
    )
    .await?;
    let d = sqlx::query_as(&format!("{SELECT} WHERE d.id = $1"))
        .bind(id)
        .fetch_one(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(Json(d))
}

/// Removes a division that nothing uses. One that has production orders is refused with 409
/// Conflict: deactivate it instead.
#[utoipa::path(delete, path = "/production/divisions/{id}", tag = "production", params(("id" = Uuid, Path)),
    responses((status = 204), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody)))]
async fn delete_division(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    user.require(Permission::ProductionAdmin)?;
    let mut tx = state.db.begin().await?;
    let used: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM production.production_orders WHERE division_id = $1)",
    )
    .bind(id)
    .fetch_one(&mut *tx)
    .await?;
    if used {
        return Err(AppError::Conflict(
            "this division has production orders; deactivate it instead".into(),
        ));
    }
    sqlx::query(
        "DELETE FROM production.product_divisions WHERE division_id = $1 AND company_id = $2",
    )
    .bind(id)
    .bind(user.company_id)
    .execute(&mut *tx)
    .await?;
    let done = sqlx::query("DELETE FROM production.divisions WHERE id = $1 AND company_id = $2")
        .bind(id)
        .bind(user.company_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| match &e {
            // An order was released for it between the check above and the delete.
            sqlx::Error::Database(d)
                if d.code().as_deref() == Some("23503")
                    && d.table() == Some("production_orders") =>
            {
                AppError::Conflict(
                    "this division has production orders; deactivate it instead".into(),
                )
            }
            _ => AppError::from(e),
        })?;
    if done.rows_affected() == 0 {
        return Err(AppError::NotFound("division"));
    }
    audit::record(
        &mut tx,
        &user,
        "production_division",
        id,
        "deleted",
        serde_json::json!({}),
    )
    .await?;
    tx.commit().await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProductDivision {
    /// The division that normally builds the product; empty when none is set.
    pub division_id: Option<Uuid>,
}

/// The division that normally builds a product.
#[utoipa::path(get, path = "/production/products/{id}/division", tag = "production", params(("id" = Uuid, Path)),
    responses((status = 200, body = ProductDivision)))]
async fn get_product_division(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(product_id): Path<Uuid>,
) -> AppResult<Json<ProductDivision>> {
    user.require(Permission::ProductionRead)?;
    let division_id = sqlx::query_scalar(
        "SELECT division_id FROM production.product_divisions WHERE company_id = $1 AND product_id = $2",
    )
    .bind(user.company_id)
    .bind(product_id)
    .fetch_optional(&state.db)
    .await?;
    Ok(Json(ProductDivision { division_id }))
}

/// Sets (or clears) the division that normally builds a product.
#[utoipa::path(put, path = "/production/products/{id}/division", tag = "production", params(("id" = Uuid, Path)),
    request_body = ProductDivision,
    responses((status = 200, body = ProductDivision), (status = 404, body = ErrorBody), (status = 422, body = ErrorBody)))]
async fn set_product_division(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(product_id): Path<Uuid>,
    Json(input): Json<ProductDivision>,
) -> AppResult<Json<ProductDivision>> {
    user.require(Permission::ProductionAdmin)?;
    let mut tx = state.db.begin().await?;
    tradocraft_products::get_product(&mut *tx, product_id).await?;
    match input.division_id {
        Some(d) => {
            let ok: bool = sqlx::query_scalar(
                "SELECT EXISTS (SELECT 1 FROM production.divisions WHERE id = $1 AND company_id = $2 AND active)",
            )
            .bind(d)
            .bind(user.company_id)
            .fetch_one(&mut *tx)
            .await?;
            if !ok {
                return Err(AppError::Validation(
                    "the division does not exist or is not active".into(),
                ));
            }
            sqlx::query(
                "INSERT INTO production.product_divisions (company_id, product_id, division_id)
                 VALUES ($1, $2, $3)
                 ON CONFLICT (company_id, product_id) DO UPDATE SET division_id = EXCLUDED.division_id",
            )
            .bind(user.company_id)
            .bind(product_id)
            .bind(d)
            .execute(&mut *tx)
            .await?;
        }
        None => {
            sqlx::query("DELETE FROM production.product_divisions WHERE company_id = $1 AND product_id = $2")
                .bind(user.company_id)
                .bind(product_id)
                .execute(&mut *tx)
                .await?;
        }
    }
    audit::record(
        &mut tx,
        &user,
        "product",
        product_id,
        "production_division_set",
        serde_json::json!({ "division_id": input.division_id }),
    )
    .await?;
    tx.commit().await?;
    Ok(Json(input))
}

/// The division routes.
pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list_divisions, create_division))
        .routes(routes!(update_division, delete_division))
        .routes(routes!(get_product_division, set_product_division))
}
