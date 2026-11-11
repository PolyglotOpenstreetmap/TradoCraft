//! Product options (ADR 0037): CRUD, validation, required options added to a quotation as option
//! lines, and the cascade from a parent line to its option lines.

mod common;

use axum::http::{Method, StatusCode};
use common::{call, dec, setup};
use rust_decimal_macros::dec;
use serde_json::{json, Value};
use sqlx::PgPool;

const ADMIN: &str = "admin@example.com";
const SALES: &str = "sales@example.com";
const ALPHA: &str = "00000000-0000-7000-8000-000000000401";
const UNIT: &str = "00000000-0000-7000-8000-000000000301";
const PUMP: &str = "00000000-0000-7000-8000-000000000303";
const FRAME: &str = "00000000-0000-7000-8000-000000000307";
const BOLT: &str = "00000000-0000-7000-8000-000000000309";

/// PUTs the option list of the unit product as `user`.
async fn put_options(app: &axum::Router, user: &str, body: Value) -> (StatusCode, Value) {
    call(
        app,
        user,
        Method::PUT,
        &format!("/api/v1/products/{UNIT}/options"),
        Some(body),
    )
    .await
}

/// Creates a quotation with the given lines.
async fn quote(app: &axum::Router, lines: Value) -> (StatusCode, Value) {
    call(
        app,
        SALES,
        Method::POST,
        "/api/v1/quotations",
        Some(json!({ "customer_id": ALPHA, "lines": lines })),
    )
    .await
}

/// The product codes of the quotation lines, in order.
fn codes(q: &Value) -> Vec<&str> {
    q["lines"]
        .as_array()
        .unwrap()
        .iter()
        .map(|l| l["product_code"].as_str().unwrap())
        .collect()
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn options_can_be_replaced_and_listed(db: PgPool) {
    let audit_db = db.clone();
    let app = setup(db).await;
    let uri = format!("/api/v1/products/{UNIT}/options");
    let (status, list) = call(&app, SALES, Method::GET, &uri, None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(list, json!([]));

    let (status, list) = put_options(
        &app,
        ADMIN,
        json!([
            { "option_product_id": FRAME, "required": true },
            { "option_product_id": PUMP, "default_quantity": "2", "required": true },
            { "option_product_id": BOLT, "default_quantity": "8" },
        ]),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{list}");
    let list = list.as_array().unwrap();
    assert_eq!(list.len(), 3);
    assert_eq!(list[0]["option_code"], "FRAME-A");
    assert_eq!(list[0]["required"], true);
    assert_eq!(dec(&list[0]["default_quantity"]), dec!(1));
    assert_eq!(dec(&list[1]["default_quantity"]), dec!(2));
    assert_eq!(list[2]["required"], false);
    let (_, again) = call(&app, SALES, Method::GET, &uri, None).await;
    assert_eq!(again.as_array().unwrap().len(), 3);

    // The set is replaced, not merged.
    let (status, list) = put_options(&app, ADMIN, json!([{ "option_product_id": BOLT }])).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(list.as_array().unwrap().len(), 1);

    // The audit trail records the change.
    let audited: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM audit.events
         WHERE entity = 'product' AND entity_id = $1::uuid AND action = 'options_replaced'",
    )
    .bind(UNIT)
    .fetch_one(&audit_db)
    .await
    .unwrap();
    assert!(audited >= 1);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn options_are_validated_and_need_the_permission(db: PgPool) {
    let app = setup(db).await;
    for body in [
        json!([{ "option_product_id": UNIT }]),
        json!([{ "option_product_id": BOLT }, { "option_product_id": BOLT }]),
        json!([{ "option_product_id": BOLT, "default_quantity": "0" }]),
        json!([{ "option_product_id": BOLT, "default_quantity": "0.00004" }]),
        json!([{ "option_product_id": "00000000-0000-7000-8000-0000000009ff" }]),
    ] {
        let (status, e) = put_options(&app, ADMIN, body.clone()).await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body} {e}");
    }
    let (status, _) = put_options(&app, "support@example.com", json!([])).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    let (status, _) = call(
        &app,
        ADMIN,
        Method::GET,
        "/api/v1/products/00000000-0000-7000-8000-0000000009ff/options",
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn required_options_are_added_as_child_lines(db: PgPool) {
    let app = setup(db.clone()).await;
    let (status, _) = put_options(
        &app,
        ADMIN,
        json!([
            { "option_product_id": FRAME, "required": true },
            { "option_product_id": PUMP, "default_quantity": "2", "required": true },
            { "option_product_id": BOLT, "default_quantity": "8" },
        ]),
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    // Without the flag nothing is added; with it, the required options follow their parent.
    let (status, plain) = quote(&app, json!([{ "product_id": UNIT, "quantity": "3" }])).await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(codes(&plain), ["UNIT-100"]);

    let (status, q) = quote(
        &app,
        json!([
            { "product_id": UNIT, "quantity": "3", "add_options": true },
            { "product_id": PUMP, "quantity": "1" },
        ]),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{q}");
    // The optional bolt is not added; the pump line the user typed stays top level, after them.
    assert_eq!(codes(&q), ["UNIT-100", "FRAME-A", "PUMP-05", "PUMP-05"]);
    let lines = q["lines"].as_array().unwrap();
    assert_eq!(lines[0]["parent_position"], Value::Null);
    assert_eq!(lines[1]["parent_position"], 1);
    assert_eq!(lines[2]["parent_position"], 1);
    assert_eq!(lines[3]["parent_position"], Value::Null);
    assert_eq!(dec(&lines[1]["quantity"]), dec!(3));
    assert_eq!(dec(&lines[2]["quantity"]), dec!(6));
    // Option lines are priced as normal products, and the flag is not stored: reading shows the
    // same lines and saving them again does not add anything.
    assert!(dec(&lines[1]["line_total"]) > dec!(0));
    let id = q["id"].as_str().unwrap();
    let (_, got) = call(
        &app,
        SALES,
        Method::GET,
        &format!("/api/v1/quotations/{id}"),
        None,
    )
    .await;
    assert_eq!(codes(&got), codes(&q));

    // An option that is already a child of the line is not added a second time.
    let (status, q) = quote(
        &app,
        json!([
            { "product_id": UNIT, "quantity": "1", "add_options": true },
            { "product_id": FRAME, "quantity": "5", "parent_position": 1 },
        ]),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{q}");
    // The missing option comes first, the line the user typed keeps its quantity.
    assert_eq!(codes(&q), ["UNIT-100", "PUMP-05", "FRAME-A"]);
    assert_eq!(dec(&q["lines"][2]["quantity"]), dec!(5));
    assert_eq!(q["lines"][1]["parent_position"], 1);
    assert_eq!(q["lines"][2]["parent_position"], 1);

    // Inactive options are skipped.
    sqlx::query("UPDATE products.product_options SET active = false WHERE option_product_id = $1")
        .bind(uuid::Uuid::parse_str(PUMP).unwrap())
        .execute(&db)
        .await
        .unwrap();
    let (_, q) = quote(
        &app,
        json!([{ "product_id": UNIT, "quantity": "1", "add_options": true }]),
    )
    .await;
    assert_eq!(codes(&q), ["UNIT-100", "FRAME-A"]);

    // The preview expands the options too.
    let (status, p) = call(
        &app,
        SALES,
        Method::POST,
        "/api/v1/pricing/preview",
        Some(json!({ "customer_id": ALPHA,
            "lines": [{ "product_id": UNIT, "quantity": "1", "add_options": true }] })),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{p}");
    assert_eq!(codes(&p), ["UNIT-100", "FRAME-A"]);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn removing_a_parent_line_removes_its_option_lines(db: PgPool) {
    let app = setup(db.clone()).await;
    put_options(
        &app,
        ADMIN,
        json!([{ "option_product_id": FRAME, "required": true }]),
    )
    .await;
    let (_, q) = quote(
        &app,
        json!([{ "product_id": UNIT, "quantity": "1", "add_options": true }]),
    )
    .await;
    let id = q["id"].as_str().unwrap();
    assert_eq!(codes(&q), ["UNIT-100", "FRAME-A"]);
    // Deleting the parent row takes the option line with it.
    let removed = sqlx::query(
        "DELETE FROM sales.quotation_lines WHERE quotation_id = $1::uuid AND position = 1",
    )
    .bind(id)
    .execute(&db)
    .await
    .unwrap();
    assert_eq!(removed.rows_affected(), 1);
    let left: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM sales.quotation_lines WHERE quotation_id = $1::uuid",
    )
    .bind(id)
    .fetch_one(&db)
    .await
    .unwrap();
    assert_eq!(left, 0);
}
