//! Delivery settings per company: what happens when a delivery is planned on a day the customer is
//! closed. No row means the defaults (`warn`).

use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgConnection};
use tradocraft_core::{audit, AppError, AppResult, AppState, CurrentUser, Permission};
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use uuid::Uuid;

type ErrorBody = tradocraft_core::error::ErrorBody;

/// The closure policies, as stored.
pub const POLICIES: [&str; 3] = ["warn", "override", "block"];

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct DeliverySettings {
    /// warn: the date is saved and the closure is shown as a warning (default) |
    /// override: needs `delivery.override_closure` and a reason | block: always refused
    pub closure_policy: String,
}

impl Default for DeliverySettings {
    fn default() -> Self {
        DeliverySettings {
            closure_policy: "warn".into(),
        }
    }
}

/// The settings of a company; the defaults while none are saved.
pub async fn load_settings(
    conn: &mut PgConnection,
    company_id: Uuid,
) -> AppResult<DeliverySettings> {
    Ok(
        sqlx::query_as("SELECT closure_policy FROM delivery.settings WHERE company_id = $1")
            .bind(company_id)
            .fetch_optional(conn)
            .await?
            .unwrap_or_default(),
    )
}

/// The delivery settings of the company.
#[utoipa::path(get, path = "/delivery/settings", tag = "delivery",
    responses((status = 200, body = DeliverySettings)))]
async fn get_settings(
    State(state): State<AppState>,
    user: CurrentUser,
) -> AppResult<Json<DeliverySettings>> {
    user.require(Permission::DeliveryRead)?;
    let mut conn = state.db.acquire().await?;
    Ok(Json(load_settings(&mut conn, user.company_id).await?))
}

/// Changes the delivery settings. Audited.
#[utoipa::path(put, path = "/delivery/settings", tag = "delivery", request_body = DeliverySettings,
    responses((status = 200, body = DeliverySettings), (status = 422, body = ErrorBody)))]
async fn put_settings(
    State(state): State<AppState>,
    user: CurrentUser,
    Json(input): Json<DeliverySettings>,
) -> AppResult<Json<DeliverySettings>> {
    user.require(Permission::DeliveryAdmin)?;
    if !POLICIES.contains(&input.closure_policy.as_str()) {
        return Err(AppError::Validation(format!(
            "closure_policy must be one of {}",
            POLICIES.join(", ")
        )));
    }
    let mut tx = state.db.begin().await?;
    // One update at a time per company, also while no row exists yet, so `before` is never stale.
    sqlx::query(
        "SELECT pg_advisory_xact_lock(hashtextextended('delivery.settings:' || $1::text, 0))",
    )
    .bind(user.company_id)
    .execute(&mut *tx)
    .await?;
    let before = load_settings(&mut tx, user.company_id).await?;
    sqlx::query(
        "INSERT INTO delivery.settings (company_id, closure_policy) VALUES ($1, $2)
         ON CONFLICT (company_id) DO UPDATE SET closure_policy = EXCLUDED.closure_policy, updated_at = now()",
    )
    .bind(user.company_id)
    .bind(&input.closure_policy)
    .execute(&mut *tx)
    .await?;
    audit::record(
        &mut tx,
        &user,
        "delivery_settings",
        user.company_id,
        "settings_updated",
        serde_json::json!({ "closure_policy": { "from": before.closure_policy, "to": input.closure_policy } }),
    )
    .await?;
    tx.commit().await?;
    Ok(Json(input))
}

/// The settings routes.
pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(get_settings, put_settings))
}
