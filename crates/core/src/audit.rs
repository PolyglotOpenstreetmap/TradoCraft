use sqlx::PgConnection;
use uuid::Uuid;

use crate::auth::CurrentUser;

/// Appends a business event to the audit trail, in the caller's transaction.
pub async fn record(
    conn: &mut PgConnection,
    user: &CurrentUser,
    entity: &str,
    entity_id: Uuid,
    action: &str,
    data: serde_json::Value,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO audit.events (company_id, user_id, entity, entity_id, action, data)
         VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(user.company_id)
    .bind(user.user_id)
    .bind(entity)
    .bind(entity_id)
    .bind(action)
    .bind(data)
    .execute(conn)
    .await?;
    Ok(())
}

/// Appends an event that has no staff user behind it (e.g. a customer acting in the portal).
/// Put who acted in `data`.
pub async fn record_system(
    conn: &mut PgConnection,
    company_id: Uuid,
    entity: &str,
    entity_id: Uuid,
    action: &str,
    data: serde_json::Value,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO audit.events (company_id, user_id, entity, entity_id, action, data)
         VALUES ($1, NULL, $2, $3, $4, $5)",
    )
    .bind(company_id)
    .bind(entity)
    .bind(entity_id)
    .bind(action)
    .bind(data)
    .execute(conn)
    .await?;
    Ok(())
}
