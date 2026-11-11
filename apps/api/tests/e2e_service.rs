//! Service: a warranty repair for a delivered serial unit, and spare parts sold for that unit.
//!
//! Real behaviour the plan glossed over: the repair stores the serial *number* (text) and a
//! warranty value chosen by the person registering it; it is not derived from the serial unit.
//! The warranty status of the unit itself (`in_warranty` / `expired` / `unknown`) is computed from
//! `warranty_until` on the serial unit.

mod common;

use axum::http::{Method, StatusCode};
use common::{call, setup};
use serde_json::{json, Value};
use sqlx::PgPool;

const ADMIN: &str = "admin@example.com";
const SUPPORT: &str = "support@example.com";
const SALES: &str = "sales@example.com";
const ALPHA: &str = "00000000-0000-7000-8000-000000000401";
const UNIT_100: &str = "00000000-0000-7000-8000-000000000301";
const PUMP: &str = "00000000-0000-7000-8000-000000000303";
const CTRL: &str = "00000000-0000-7000-8000-000000000310";
const U100_0001: &str = "00000000-0000-7000-8000-000000000601";

async fn api(
    app: &axum::Router,
    user: &str,
    method: Method,
    uri: &str,
    body: Option<Value>,
) -> (StatusCode, Value) {
    call(app, user, method, &format!("/api/v1{uri}"), body).await
}

/// Delivers the seeded unit to Alpha and records the end of its warranty.
async fn sold_unit(app: &axum::Router, warranty_until: &str) -> Value {
    let (status, d) = api(
        app,
        ADMIN,
        Method::PUT,
        &format!("/serials/{U100_0001}/status"),
        Some(json!({ "status": "delivered", "customer_id": ALPHA })),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{d}");
    let (status, d) = api(
        app,
        ADMIN,
        Method::PUT,
        &format!("/serials/{U100_0001}/warranty"),
        Some(json!({ "warranty_until": warranty_until, "warranty_note": "24 months" })),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{d}");
    d
}

async fn repair_for_unit(app: &axum::Router, unit: &Value, warranty: &str) -> Value {
    let (status, r) = api(
        app,
        SUPPORT,
        Method::POST,
        "/repairs",
        Some(json!({
            "customer_id": unit["unit"]["customer_id"],
            "product_id": unit["unit"]["product_id"],
            "serial_number": unit["unit"]["serial_number"],
            "fault_description": "Pump does not start",
            "warranty": warranty,
        })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{r}");
    r
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn a_repair_for_a_unit_in_warranty_references_the_serial_unit(db: PgPool) {
    let app = setup(db).await;
    let unit = sold_unit(&app, "2099-12-31").await;
    assert_eq!(unit["unit"]["warranty_status"], "in_warranty");
    assert_eq!(unit["unit"]["warranty_until"], "2099-12-31");

    let r = repair_for_unit(&app, &unit, "in_warranty").await;
    assert_eq!(r["repair"]["warranty"], "in_warranty");
    assert_eq!(r["repair"]["serial_number"], "U100-0001");
    assert_eq!(r["repair"]["customer_id"], ALPHA);
    assert_eq!(r["repair"]["product_id"], UNIT_100);

    // The reference resolves: the serial number finds the very unit that was repaired.
    let (_, found) = api(&app, SUPPORT, Method::GET, "/serials?q=U100-0001", None).await;
    assert_eq!(found["items"][0]["id"], U100_0001);
    let (status, d) = api(
        &app,
        SUPPORT,
        Method::GET,
        &format!("/serials/{U100_0001}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{d}");
    assert_eq!(d["unit"]["warranty_status"], "in_warranty");
    assert_eq!(d["unit"]["serial_number"], r["repair"]["serial_number"]);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn a_unit_past_its_warranty_end_is_reported_expired(db: PgPool) {
    let app = setup(db).await;
    let unit = sold_unit(&app, "2020-01-31").await;
    assert_eq!(unit["unit"]["warranty_status"], "expired");
    let r = repair_for_unit(&app, &unit, "out_of_warranty").await;
    assert_eq!(r["repair"]["warranty"], "out_of_warranty");

    // Clearing the date makes the status unknown rather than guessing.
    let (status, d) = api(
        &app,
        ADMIN,
        Method::PUT,
        &format!("/serials/{U100_0001}/warranty"),
        Some(json!({})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{d}");
    assert_eq!(d["unit"]["warranty_status"], "unknown");
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn spare_parts_for_a_sold_unit_are_sold_through_a_quotation(db: PgPool) {
    let app = setup(db).await;
    sold_unit(&app, "2020-01-31").await;

    // Replacement pump for the unit, priced by the server.
    let (status, q) = api(
        &app,
        SALES,
        Method::POST,
        "/quotations",
        Some(json!({ "customer_id": ALPHA, "lines": [{ "product_id": PUMP, "quantity": "1" }] })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{q}");
    assert_eq!(q["lines"][0]["product_code"], "PUMP-05");
    assert_eq!(q["customer_id"], ALPHA, "sold to the owner of the unit");

    // The pump is linked to the unit's product by a compatibility rule.
    let (_, rel) = api(
        &app,
        SALES,
        Method::GET,
        &format!("/products/{UNIT_100}/related"),
        None,
    )
    .await;
    let linked = |code: &str| {
        rel["rules"].as_array().unwrap().iter().any(|r| {
            r["kind"] == "compatible"
                && [&r["a_product_code"], &r["b_product_code"]]
                    .iter()
                    .any(|c| **c == code)
        })
    };
    assert!(linked("PUMP-05"), "{rel}");
    // Gap: the controller is built into the unit (BOM) but no relation rule links it to UNIT-100.
    assert!(!linked("CTRL-01"), "{rel}");
    let (_, c) = api(
        &app,
        SALES,
        Method::POST,
        "/quotations",
        Some(json!({ "customer_id": ALPHA, "lines": [{ "product_id": CTRL, "quantity": "1" }] })),
    )
    .await;
    assert_eq!(c["lines"][0]["product_code"], "CTRL-01");
}
