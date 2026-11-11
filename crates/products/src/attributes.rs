//! Typed product attributes (ADR 0034): key → number, text or boolean, with an optional unit.
//! Configurator formulas read them (`product(code).attr('kw')`) and product pickers filter on
//! them. Shared master data like the products themselves, so no `company_id`.

use std::collections::HashMap;

use axum::extract::{Path, State};
use axum::Json;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgExecutor};
use tradocraft_core::{audit, AppError, AppResult, AppState, CurrentUser, Permission};
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use uuid::Uuid;

type ErrorBody = tradocraft_core::error::ErrorBody;

/// Longest accepted key and text value.
const MAX_KEY: usize = 64;
const MAX_TEXT: usize = 1000;
/// Most attributes per product.
const MAX_ATTRIBUTES: usize = 200;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum AttributeKind {
    Number,
    Text,
    Boolean,
}

impl AttributeKind {
    fn as_str(self) -> &'static str {
        match self {
            AttributeKind::Number => "number",
            AttributeKind::Text => "text",
            AttributeKind::Boolean => "boolean",
        }
    }
}

/// One attribute. `value` is text in JSON: a decimal for `number` (e.g. `"0.55"`), `true` or
/// `false` for `boolean`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ProductAttribute {
    /// Letters, digits and `_`, not starting with a digit, e.g. `kw`.
    pub key: String,
    pub kind: AttributeKind,
    pub value: String,
    /// Display unit, e.g. `kW`.
    #[serde(default)]
    pub unit: Option<String>,
}

/// A typed attribute value.
#[derive(Debug, Clone, PartialEq)]
pub enum AttributeValue {
    Number(Decimal),
    Text(String),
    Boolean(bool),
}

#[derive(FromRow)]
struct Row {
    product_id: Uuid,
    key: String,
    kind: String,
    value_number: Option<Decimal>,
    value_text: Option<String>,
    value_bool: Option<bool>,
    unit: Option<String>,
}

impl Row {
    fn value(&self) -> AttributeValue {
        match self.kind.as_str() {
            "number" => AttributeValue::Number(self.value_number.unwrap_or_default()),
            "boolean" => AttributeValue::Boolean(self.value_bool.unwrap_or_default()),
            _ => AttributeValue::Text(self.value_text.clone().unwrap_or_default()),
        }
    }

    fn to_api(&self) -> ProductAttribute {
        let (kind, value) = match self.value() {
            AttributeValue::Number(d) => (AttributeKind::Number, d.normalize().to_string()),
            AttributeValue::Boolean(b) => (AttributeKind::Boolean, b.to_string()),
            AttributeValue::Text(t) => (AttributeKind::Text, t),
        };
        ProductAttribute {
            key: self.key.clone(),
            kind,
            value,
            unit: self.unit.clone(),
        }
    }
}

const SELECT: &str = "SELECT product_id, key, kind, value_number, value_text, value_bool, unit
                      FROM products.product_attributes";

/// One product of the catalogue snapshot used by configurators.
#[derive(Debug, Clone)]
pub struct CatalogEntry {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub family_code: Option<String>,
    pub unit: String,
    pub active: bool,
    pub attributes: Vec<(String, AttributeValue)>,
}

/// Every product with its family code, active flag and typed attributes.
pub async fn catalog_snapshot(conn: &mut sqlx::PgConnection) -> AppResult<Vec<CatalogEntry>> {
    let products: Vec<(Uuid, String, String, Option<String>, String, bool)> = sqlx::query_as(
        "SELECT p.id, p.code, p.name, f.code, p.unit, p.active
         FROM products.products p LEFT JOIN products.product_families f ON f.id = p.family_id",
    )
    .fetch_all(&mut *conn)
    .await?;
    let rows: Vec<Row> = sqlx::query_as(&format!("{SELECT} ORDER BY product_id, key"))
        .fetch_all(&mut *conn)
        .await?;
    let mut attrs: HashMap<Uuid, Vec<(String, AttributeValue)>> = HashMap::new();
    for r in &rows {
        attrs
            .entry(r.product_id)
            .or_default()
            .push((r.key.clone(), r.value()));
    }
    Ok(products
        .into_iter()
        .map(|(id, code, name, family_code, unit, active)| CatalogEntry {
            id,
            code,
            name,
            family_code,
            unit,
            active,
            attributes: attrs.remove(&id).unwrap_or_default(),
        })
        .collect())
}

async fn ensure_product<'e>(db: impl PgExecutor<'e>, id: Uuid) -> AppResult<()> {
    let exists: bool =
        sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM products.products WHERE id = $1)")
            .bind(id)
            .fetch_one(db)
            .await?;
    if exists {
        Ok(())
    } else {
        Err(AppError::NotFound("product"))
    }
}

async fn load<'e>(db: impl PgExecutor<'e>, id: Uuid) -> AppResult<Vec<ProductAttribute>> {
    let rows: Vec<Row> = sqlx::query_as(&format!("{SELECT} WHERE product_id = $1 ORDER BY key"))
        .bind(id)
        .fetch_all(db)
        .await?;
    Ok(rows.iter().map(Row::to_api).collect())
}

fn valid_key(k: &str) -> bool {
    let mut c = k.chars();
    matches!(c.next(), Some(x) if x.is_ascii_alphabetic() || x == '_')
        && c.all(|x| x.is_ascii_alphanumeric() || x == '_')
        && k.len() <= MAX_KEY
}

/// Checks and types one attribute.
fn parse(a: &ProductAttribute) -> AppResult<AttributeValue> {
    let key = a.key.trim();
    if !valid_key(key) {
        return Err(AppError::Validation(format!(
            "'{}' is not a valid attribute key (letters, digits and _, at most {MAX_KEY})",
            a.key
        )));
    }
    let v = a.value.trim();
    Ok(match a.kind {
        AttributeKind::Number => AttributeValue::Number(v.parse().map_err(|_| {
            AppError::Validation(format!("attribute {key}: '{v}' is not a number"))
        })?),
        AttributeKind::Boolean => match v {
            "true" => AttributeValue::Boolean(true),
            "false" => AttributeValue::Boolean(false),
            _ => {
                return Err(AppError::Validation(format!(
                    "attribute {key}: a boolean is true or false"
                )))
            }
        },
        AttributeKind::Text => {
            if v.chars().count() > MAX_TEXT {
                return Err(AppError::Validation(format!(
                    "attribute {key}: at most {MAX_TEXT} characters"
                )));
            }
            AttributeValue::Text(v.to_string())
        }
    })
}

/// The typed attributes of a product.
#[utoipa::path(get, path = "/products/{id}/attributes", tag = "products", params(("id" = Uuid, Path)),
    responses((status = 200, body = Vec<ProductAttribute>), (status = 404, body = ErrorBody)))]
async fn list_attributes(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Vec<ProductAttribute>>> {
    user.require(Permission::ProductsRead)?;
    ensure_product(&state.db, id).await?;
    Ok(Json(load(&state.db, id).await?))
}

/// Replaces all attributes of a product with the given list (one entry per key).
#[utoipa::path(put, path = "/products/{id}/attributes", tag = "products", params(("id" = Uuid, Path)),
    request_body = Vec<ProductAttribute>,
    responses((status = 200, body = Vec<ProductAttribute>), (status = 404, body = ErrorBody),
        (status = 422, body = ErrorBody)))]
async fn replace_attributes(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<Uuid>,
    Json(input): Json<Vec<ProductAttribute>>,
) -> AppResult<Json<Vec<ProductAttribute>>> {
    user.require(Permission::ProductsWrite)?;
    if input.len() > MAX_ATTRIBUTES {
        return Err(AppError::Validation(format!(
            "a product has at most {MAX_ATTRIBUTES} attributes"
        )));
    }
    let mut typed = Vec::with_capacity(input.len());
    let mut keys: Vec<String> = Vec::new();
    for a in &input {
        let v = parse(a)?;
        let key = a.key.trim().to_string();
        if keys.contains(&key) {
            return Err(AppError::Validation(format!(
                "attribute {key} is given twice"
            )));
        }
        keys.push(key.clone());
        let unit = a
            .unit
            .as_deref()
            .map(str::trim)
            .filter(|u| !u.is_empty())
            .map(str::to_string);
        typed.push((key, a.kind, v, unit));
    }
    let mut tx = state.db.begin().await?;
    ensure_product(&mut *tx, id).await?;
    let before = load(&mut *tx, id).await?;
    sqlx::query("DELETE FROM products.product_attributes WHERE product_id = $1")
        .bind(id)
        .execute(&mut *tx)
        .await?;
    for (key, kind, v, unit) in &typed {
        let (n, t, b) = match v {
            AttributeValue::Number(d) => (Some(*d), None, None),
            AttributeValue::Text(s) => (None, Some(s.clone()), None),
            AttributeValue::Boolean(x) => (None, None, Some(*x)),
        };
        sqlx::query(
            "INSERT INTO products.product_attributes
                 (product_id, key, kind, value_number, value_text, value_bool, unit)
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(id)
        .bind(key)
        .bind(kind.as_str())
        .bind(n)
        .bind(t)
        .bind(b)
        .bind(unit)
        .execute(&mut *tx)
        .await?;
    }
    let after = load(&mut *tx, id).await?;
    audit::record(
        &mut tx,
        &user,
        "product",
        id,
        "attributes_replaced",
        serde_json::json!({ "before": before, "after": after }),
    )
    .await?;
    tx.commit().await?;
    Ok(Json(after))
}

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(list_attributes, replace_attributes))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn a(key: &str, kind: AttributeKind, value: &str) -> ProductAttribute {
        ProductAttribute {
            key: key.into(),
            kind,
            value: value.into(),
            unit: None,
        }
    }

    #[test]
    fn parses_typed_values_and_rejects_bad_ones() {
        assert_eq!(
            parse(&a("kw", AttributeKind::Number, "0.55")).unwrap(),
            AttributeValue::Number("0.55".parse().unwrap())
        );
        assert_eq!(
            parse(&a("ip", AttributeKind::Boolean, "true")).unwrap(),
            AttributeValue::Boolean(true)
        );
        assert!(parse(&a("kw", AttributeKind::Number, "fast")).is_err());
        assert!(parse(&a("1kw", AttributeKind::Text, "x")).is_err());
        assert!(parse(&a("ok", AttributeKind::Boolean, "yes")).is_err());
    }
}
