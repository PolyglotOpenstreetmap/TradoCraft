//! Opportunities and their link with quotations.

mod common;

use axum::http::{Method, StatusCode};
use common::{call, dec, setup};
use serde_json::json;
use sqlx::PgPool;

const ALPHA: &str = "00000000-0000-7000-8000-000000000401";
const BETA: &str = "00000000-0000-7000-8000-000000000402";
const UNIT_100: &str = "00000000-0000-7000-8000-000000000301";
const OPP_ALPHA: &str = "00000000-0000-7000-8000-000000000701"; // qualified
const SALES: &str = "sales@example.com";

fn quote(customer: &str, opportunity: &str) -> serde_json::Value {
    json!({
        "customer_id": customer,
        "opportunity_id": opportunity,
        "lines": [{ "product_id": UNIT_100, "quantity": "1" }]
    })
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn new_opportunity_starts_as_lead(db: PgPool) {
    let app = setup(db).await;
    let (status, o) = call(
        &app,
        SALES,
        Method::POST,
        "/api/v1/opportunities",
        Some(json!({ "title": "New site", "customer_id": BETA, "expected_value": "12000", "expected_close": "2027-03-01" })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{o}");
    assert!(o["number"].as_str().unwrap().starts_with('O'));
    assert_eq!(o["stage"], "lead");
    assert_eq!(o["probability"], 10);
    assert_eq!(o["owner_name"], "Sales User");
    assert_eq!(o["expected_close"], "2027-03-01");
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn accepting_a_linked_quotation_wins_the_opportunity(db: PgPool) {
    let app = setup(db).await;
    let (status, q) = call(
        &app,
        SALES,
        Method::POST,
        "/api/v1/quotations",
        Some(quote(ALPHA, OPP_ALPHA)),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{q}");
    assert_eq!(q["opportunity_id"], OPP_ALPHA);

    // A quotation moves an early-stage opportunity to "proposal".
    let (_, o) = call(
        &app,
        SALES,
        Method::GET,
        &format!("/api/v1/opportunities/{OPP_ALPHA}"),
        None,
    )
    .await;
    assert_eq!(o["stage"], "proposal");
    assert_eq!(o["probability"], 50);

    let id = q["id"].as_str().unwrap();
    call(
        &app,
        SALES,
        Method::POST,
        &format!("/api/v1/quotations/{id}/send"),
        None,
    )
    .await;
    let (status, _) = call(
        &app,
        SALES,
        Method::POST,
        &format!("/api/v1/quotations/{id}/accept"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    let (_, o) = call(
        &app,
        SALES,
        Method::GET,
        &format!("/api/v1/opportunities/{OPP_ALPHA}"),
        None,
    )
    .await;
    assert_eq!(o["stage"], "won");
    assert_eq!(o["probability"], 100);
    assert!(o["closed_at"].is_string());

    // Quotations of an opportunity can be listed.
    let (_, page) = call(
        &app,
        SALES,
        Method::GET,
        &format!("/api/v1/quotations?opportunity_id={OPP_ALPHA}"),
        None,
    )
    .await;
    assert_eq!(page["total"], 1);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn quotation_must_match_an_open_opportunity_of_the_same_customer(db: PgPool) {
    let app = setup(db).await;
    let (status, _) = call(
        &app,
        SALES,
        Method::POST,
        "/api/v1/quotations",
        Some(quote(BETA, OPP_ALPHA)),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);

    let stage = format!("/api/v1/opportunities/{OPP_ALPHA}/stage");
    let (status, _) = call(
        &app,
        SALES,
        Method::POST,
        &stage,
        Some(json!({ "stage": "lost" })),
    )
    .await;
    assert_eq!(
        status,
        StatusCode::UNPROCESSABLE_ENTITY,
        "lost needs a reason"
    );
    let (status, o) = call(
        &app,
        SALES,
        Method::POST,
        &stage,
        Some(json!({ "stage": "lost", "lost_reason": "Price" })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(o["lost_reason"], "Price");
    assert_eq!(o["probability"], 0);

    let (status, _) = call(
        &app,
        SALES,
        Method::POST,
        "/api/v1/quotations",
        Some(quote(ALPHA, OPP_ALPHA)),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);

    // Reopening clears the closing data.
    let (_, o) = call(
        &app,
        SALES,
        Method::POST,
        &stage,
        Some(json!({ "stage": "negotiation" })),
    )
    .await;
    assert!(o["closed_at"].is_null());
    assert!(o["lost_reason"].is_null());
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn customer_of_an_opportunity_is_fixed(db: PgPool) {
    let app = setup(db).await;
    let (status, _) = call(
        &app,
        SALES,
        Method::PUT,
        &format!("/api/v1/opportunities/{OPP_ALPHA}"),
        Some(json!({ "title": "Second production line", "customer_id": BETA })),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn pipeline_reports_every_stage_with_weighted_value(db: PgPool) {
    let app = setup(db).await;
    let (status, p) = call(
        &app,
        "viewer@example.com",
        Method::GET,
        "/api/v1/opportunities/pipeline",
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let stages: Vec<_> = p
        .as_array()
        .unwrap()
        .iter()
        .map(|s| s["stage"].as_str().unwrap())
        .collect();
    assert_eq!(
        stages,
        [
            "lead",
            "qualified",
            "proposal",
            "negotiation",
            "won",
            "lost"
        ]
    );
    let negotiation = &p[3];
    assert_eq!(negotiation["count"], 1);
    assert_eq!(dec(&negotiation["total_value"]), "65000".parse().unwrap());
    assert_eq!(
        dec(&negotiation["weighted_value"]),
        "48750".parse().unwrap()
    );

    let (status, _) = call(
        &app,
        "viewer@example.com",
        Method::POST,
        "/api/v1/opportunities",
        Some(json!({ "title": "x", "customer_id": ALPHA })),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
}
