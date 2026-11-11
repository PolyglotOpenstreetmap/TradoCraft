//! Products: pick products that belong together through the relation rules, check each against
//! the destination power grid, and see conflicting products flagged in the pricing preview.
//!
//! Deviation from the plan: "incompatible pair" is flagged as a warning of kind
//! `incompatible_products` (never an error), and the power result of a quotation is a
//! `power_grid` warning; the per-product answer comes from `power-compatibility`.

mod common;

use axum::http::{Method, StatusCode};
use common::{call, setup};
use serde_json::{json, Value};
use sqlx::PgPool;

const SALES: &str = "sales@example.com";
const ALPHA: &str = "00000000-0000-7000-8000-000000000401";
const UNIT_100: &str = "00000000-0000-7000-8000-000000000301";
const UNIT_200: &str = "00000000-0000-7000-8000-000000000302";
const PUMP: &str = "00000000-0000-7000-8000-000000000303";
const HOSE: &str = "00000000-0000-7000-8000-000000000304";
const PROFILE: &str = "00000000-0000-7000-8000-000000000308";
const CTRL: &str = "00000000-0000-7000-8000-000000000310";

async fn get(app: &axum::Router, uri: &str) -> (StatusCode, Value) {
    call(app, SALES, Method::GET, &format!("/api/v1{uri}"), None).await
}

async fn preview(app: &axum::Router, products: &[&str], extra: Value) -> Value {
    let lines: Vec<Value> = products
        .iter()
        .map(|p| json!({ "product_id": p, "quantity": "1" }))
        .collect();
    let mut body = json!({ "customer_id": ALPHA, "lines": lines });
    for (k, v) in extra.as_object().unwrap() {
        body[k] = v.clone();
    }
    let (status, p) = call(
        app,
        SALES,
        Method::POST,
        "/api/v1/pricing/preview",
        Some(body),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{p}");
    p
}

fn warnings_of<'a>(p: &'a Value, kind: &str) -> Vec<&'a Value> {
    p["warnings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|w| w["kind"] == kind)
        .collect()
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn related_products_are_compatible_and_fit_the_destination_grid(db: PgPool) {
    let app = setup(db).await;

    // The relation rules say the unit is designed around the pump.
    let (status, rules) = get(&app, "/product-relations?kind=compatible").await;
    assert_eq!(status, StatusCode::OK, "{rules}");
    let pair = rules
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["a_product_code"] == "UNIT-100" && r["b_product_code"] == "PUMP-05")
        .expect("UNIT-100 <-> PUMP-05 is a seeded compatibility rule");
    assert_eq!(pair["bidirectional"], true);
    let selected = [UNIT_100, PUMP];

    // From either side the rule shows up in the related products of a selected product.
    for (product, other) in [(UNIT_100, "PUMP-05"), (PUMP, "UNIT-100")] {
        let (status, rel) = get(&app, &format!("/products/{product}/related")).await;
        assert_eq!(status, StatusCode::OK, "{rel}");
        let compatible: Vec<&str> = rel["rules"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|r| r["kind"] == "compatible")
            .flat_map(|r| [r["a_product_code"].as_str(), r["b_product_code"].as_str()])
            .flatten()
            .collect();
        assert!(compatible.contains(&other), "{product}: {compatible:?}");
    }

    // The selection raises no conflict in a quotation preview...
    let p = preview(&app, &selected, json!({ "delivery_country_code": "NO" })).await;
    assert!(warnings_of(&p, "incompatible_products").is_empty(), "{p}");
    // ...and both products fit the Norwegian grid.
    for product in selected {
        let (status, r) = get(
            &app,
            &format!("/products/{product}/power-compatibility?country=NO"),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{r}");
        assert_eq!(r["status"], "compatible", "{r}");
        assert!(!r["available"].as_array().unwrap().is_empty());
    }
    assert!(
        warnings_of(&p, "power_grid").is_empty(),
        "no grid warning for a fitting selection"
    );
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn a_product_that_does_not_fit_the_grid_is_flagged(db: PgPool) {
    let app = setup(db).await;
    // UNIT-200 runs on 120V/60Hz; Norway has 230V/400V at 50Hz.
    let (_, r) = get(
        &app,
        &format!("/products/{UNIT_200}/power-compatibility?country=NO"),
    )
    .await;
    assert_eq!(r["status"], "incompatible", "{r}");
    assert_eq!(r["supported"][0]["voltage_v"], 120);
    // The controller is DC only and needs an external supply.
    let (_, r) = get(
        &app,
        &format!("/products/{CTRL}/power-compatibility?country=NO"),
    )
    .await;
    assert_eq!(r["status"], "needs_external_supply", "{r}");
    // A country without grid data is reported as unknown rather than guessed.
    let (_, r) = get(
        &app,
        &format!("/products/{UNIT_100}/power-compatibility?country=ZZ"),
    )
    .await;
    assert_eq!(r["status"], "unknown_grid", "{r}");

    // The same finding reaches the preview as an advisory warning; stock purchases skip it.
    let p = preview(
        &app,
        &[UNIT_100, UNIT_200],
        json!({ "delivery_country_code": "NO" }),
    )
    .await;
    let w = warnings_of(&p, "power_grid");
    assert_eq!(w.len(), 1, "{p}");
    assert_eq!(w[0]["product_code"], "UNIT-200");
    assert_eq!(w[0]["details"]["status"], "incompatible");
    let p = preview(&app, &[UNIT_200], json!({ "purpose": "stock" })).await;
    assert!(warnings_of(&p, "power_grid").is_empty());
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn conflicting_products_are_reported_as_an_incompatible_pair_in_the_preview(db: PgPool) {
    let app = setup(db).await;
    let p = preview(&app, &[HOSE, PROFILE], json!({})).await;
    let w = warnings_of(&p, "incompatible_products");
    assert_eq!(w.len(), 1, "{p}");
    assert_eq!(
        w[0]["details"]["product_codes"],
        json!(["HOSE-25", "PROFILE-40"])
    );
    assert_eq!(
        w[0]["details"]["note"],
        "The hose clamps do not fit the 40x40 profile"
    );
    // Warnings never block: the preview still prices both lines.
    assert_eq!(p["lines"].as_array().unwrap().len(), 2);

    // A pump suggests its controller (a rule-based suggestion, not a requirement).
    let p = preview(&app, &[PUMP], json!({})).await;
    let suggested: Vec<&str> = p["suggestions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| s["product_code"].as_str().unwrap())
        .collect();
    assert!(suggested.contains(&"CTRL-01"), "{suggested:?}");
}
