use sqlx::PgConnection;
use uuid::Uuid;

/// Allocates the next number of a per-company, per-year sequence.
///
/// Runs inside the caller's transaction: the row lock makes numbering gapless
/// and serialised; if the transaction rolls back, the number is not consumed.
pub async fn next_number(
    conn: &mut PgConnection,
    company_id: Uuid,
    code: &str,
    year: i32,
) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar(
        r#"
        INSERT INTO core.number_sequences (company_id, code, year, next_value)
        VALUES ($1, $2, $3, 2)
        ON CONFLICT (company_id, code, year)
        DO UPDATE SET next_value = core.number_sequences.next_value + 1
        RETURNING next_value - 1
        "#,
    )
    .bind(company_id)
    .bind(code)
    .bind(year)
    .fetch_one(conn)
    .await
}
