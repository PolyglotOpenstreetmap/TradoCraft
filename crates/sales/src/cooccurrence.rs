//! "Often bought together": counts which products appear on the same orders and stores the result
//! in `products.product_cooccurrence` (ADR 0027). Run by the worker.

use sqlx::PgPool;
use tradocraft_core::AppResult;
use tradocraft_products::relations::{count_pairs, replace_cooccurrence};
use uuid::Uuid;

/// The product sets of all confirmed, not cancelled orders with purpose `installation`, over all
/// companies (products are shared master data). Only the latest revision's lines count, and only
/// real goods: lines without a product (SHIPPING) and services are left out.
pub async fn installation_order_product_sets(db: &PgPool) -> AppResult<Vec<Vec<Uuid>>> {
    let rows: Vec<(Uuid, Uuid)> = sqlx::query_as(
        "SELECT o.id, l.product_id
         FROM sales.orders o
         JOIN sales.order_lines l ON l.order_id = o.id AND l.revision = o.current_revision
         JOIN products.products p ON p.id = l.product_id
         WHERE o.purpose = 'installation' AND o.confirmed_at IS NOT NULL AND o.status <> 'cancelled'
           AND p.product_type <> 'service'
         ORDER BY o.id",
    )
    .fetch_all(db)
    .await?;
    let mut sets: Vec<Vec<Uuid>> = Vec::new();
    let mut current: Option<Uuid> = None;
    for (order, product) in rows {
        if current != Some(order) {
            sets.push(Vec::new());
            current = Some(order);
        }
        if let Some(set) = sets.last_mut() {
            set.push(product);
        }
    }
    Ok(sets)
}

/// Recomputes the whole table; idempotent, and the old content is replaced in one transaction.
/// Returns the number of pairs stored.
pub async fn recompute(db: &PgPool) -> AppResult<usize> {
    let sets = installation_order_product_sets(db).await?;
    replace_cooccurrence(db, &count_pairs(&sets)).await
}
