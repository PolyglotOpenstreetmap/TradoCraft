//! Logistics: an international installation order with a quoted shipping cost, and the
//! installation job scheduled for the same customer.
//!
//! Deviations from the plan: there is no "delivery country on order send"; purpose and delivery
//! country are set on the quotation and carried to the order. Jobs have no order reference, so
//! the link to the order is the customer (and an optional ticket).

mod common;

use axum::http::{Method, StatusCode};
use common::{call, dec, setup};
use serde_json::{json, Value};
use sqlx::PgPool;

const SALES: &str = "sales@example.com";
const LOGISTICS: &str = "logistics@example.com";
const DISPATCHER: &str = "dispatcher@example.com";
const ADMIN: &str = "admin@example.com";
const GAMMA: &str = "00000000-0000-7000-8000-000000000403"; // Germany
const UNIT_100: &str = "00000000-0000-7000-8000-000000000301";
const PANEL: &str = "00000000-0000-7000-8000-00000000030b"; // needs a shipping quote
const TOM: &str = "00000000-0000-7000-8000-000000000107";

async fn api(
    app: &axum::Router,
    user: &str,
    method: Method,
    uri: &str,
    body: Option<Value>,
) -> (StatusCode, Value) {
    call(app, user, method, &format!("/api/v1{uri}"), body).await
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn international_order_carries_purpose_and_destination_and_gets_an_installation_job(
    db: PgPool,
) {
    let app = setup(db).await;
    let (status, q) = api(
        &app,
        SALES,
        Method::POST,
        "/quotations",
        Some(json!({ "customer_id": GAMMA, "purpose": "installation", "delivery_country_code": "no",
            "lines": [{ "product_id": UNIT_100, "quantity": "1" }, { "product_id": PANEL, "quantity": "1" }] })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{q}");
    assert_eq!(q["purpose"], "installation");
    assert_eq!(
        q["delivery_country_code"], "NO",
        "upper-cased, differs from DE and BE"
    );
    assert_eq!(
        q["shipping_status"], "pending",
        "the crated panel needs a quote"
    );
    let qid = q["id"].as_str().unwrap().to_string();

    // Sending waits for logistics to quote the shipping cost.
    let (status, _) = api(
        &app,
        SALES,
        Method::POST,
        &format!("/quotations/{qid}/send"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);
    let (_, queue) = api(&app, LOGISTICS, Method::GET, "/shipping-requests", None).await;
    let rid = queue[0]["id"].as_str().unwrap().to_string();
    let (status, r) = api(
        &app,
        LOGISTICS,
        Method::POST,
        &format!("/shipping-requests/{rid}/quote"),
        Some(json!({ "amount": "450" })),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{r}");
    let (status, s) = api(
        &app,
        SALES,
        Method::POST,
        &format!("/quotations/{qid}/send"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{s}");
    assert_eq!(s["shipping_status"], "quoted");
    let shipping = s["lines"]
        .as_array()
        .unwrap()
        .iter()
        .find(|l| l["product_code"] == "SHIPPING")
        .expect("the quoted cost is a line of the quotation");
    assert_eq!(dec(&shipping["line_total"]).to_string(), "450.00");

    // The order records purpose and destination.
    let (status, o) = api(
        &app,
        SALES,
        Method::POST,
        &format!("/quotations/{qid}/convert"),
        Some(json!({ "customer_reference": "PO-INT-1" })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{o}");
    assert_eq!(o["order"]["purpose"], "installation");
    assert_eq!(o["order"]["delivery_country_code"], "NO");
    assert_eq!(o["order"]["customer_id"], GAMMA);
    assert!(
        o["lines"]
            .as_array()
            .unwrap()
            .iter()
            .any(|l| l["product_code"] == "SHIPPING"),
        "{o}"
    );

    // Installation: a job for the same customer, scheduled for a technician on a Monday morning.
    let (status, _) = api(
        &app,
        ADMIN,
        Method::PUT,
        "/planning/working-hours",
        Some(json!({ "slots": (1..=5).map(|w| json!({ "weekday": w, "start_time": "08:00", "end_time": "17:00" })).collect::<Vec<_>>() })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let (status, j) = api(
        &app,
        DISPATCHER,
        Method::POST,
        "/field/jobs",
        Some(json!({ "title": "Install process unit", "kind": "installation", "customer_id": GAMMA })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{j}");
    let job = j["job"]["id"].as_str().unwrap().to_string();
    let (status, s) = api(
        &app,
        DISPATCHER,
        Method::PUT,
        &format!("/planning/jobs/{job}/schedule"),
        Some(json!({ "scheduled_start": "2026-11-09T08:00:00Z", "scheduled_end": "2026-11-09T12:00:00Z",
                     "technician_ids": [TOM] })),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{s}");
    assert_eq!(s["applied"], true);

    let (_, d) = api(
        &app,
        DISPATCHER,
        Method::GET,
        &format!("/field/jobs/{job}"),
        None,
    )
    .await;
    assert_eq!(d["job"]["status"], "planned");
    assert_eq!(d["job"]["kind"], "installation");
    assert_eq!(d["job"]["technicians"][0]["user_id"], TOM);
    assert_eq!(d["job"]["scheduled_start"], "2026-11-09T08:00:00Z");
    assert_eq!(d["job"]["scheduled_end"], "2026-11-09T12:00:00Z");
    assert_eq!(
        d["job"]["customer_id"], o["order"]["customer_id"],
        "the job is for the customer of the order"
    );
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn a_stock_purchase_has_no_destination_and_needs_no_installation(db: PgPool) {
    let app = setup(db).await;
    let (status, q) = api(
        &app,
        SALES,
        Method::POST,
        "/quotations",
        Some(json!({ "customer_id": GAMMA, "purpose": "stock",
            "lines": [{ "product_id": UNIT_100, "quantity": "1" }] })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{q}");
    assert_eq!(q["purpose"], "stock");
    assert!(q["delivery_country_code"].is_null());
    let qid = q["id"].as_str().unwrap();
    api(
        &app,
        SALES,
        Method::POST,
        &format!("/quotations/{qid}/send"),
        None,
    )
    .await;
    let (_, o) = api(
        &app,
        SALES,
        Method::POST,
        &format!("/quotations/{qid}/convert"),
        Some(json!({})),
    )
    .await;
    assert_eq!(o["order"]["purpose"], "stock");
    assert!(o["order"]["delivery_country_code"].is_null());
}
