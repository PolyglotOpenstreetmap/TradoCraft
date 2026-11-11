//! CRM: register an industrial customer, turn an unknown sender's enquiry into a lead, link it to
//! the customer and convert it to an opportunity.

mod common;

use axum::http::{Method, StatusCode};
use common::e2e::*;
use common::{call, setup};
use serde_json::json;
use sqlx::PgPool;

const SALES: &str = "sales@example.com";
const SUPPORT: &str = "support@example.com";
const ZONE_INDUSTRY: &str = "00000000-0000-7000-8000-000000000203";

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn enquiry_from_an_unknown_sender_becomes_a_lead_and_an_opportunity(db: PgPool) {
    let app = setup(db).await;

    // The unknown sender writes first: a ticket in the lead category, no customer yet.
    let r = import_email(
        &app,
        SUPPORT,
        build_email(
            "Kari Nordmann <kari@gmail.com>",
            "Quotation request",
            "Hello, we need a quotation for a process unit and two pumps. Thank you and kind regards",
            "Message-ID: <crm-1@gmail.com>\r\n",
        ),
    )
    .await;
    assert_eq!(r["new_ticket"], true, "{r}");
    let tid = r["ticket_id"].as_str().unwrap().to_string();
    let t = get_ticket(&app, SUPPORT, &tid).await;
    assert!(t["ticket"]["customer_id"].is_null());
    assert_eq!(t["ticket"]["category"], "lead");

    // A lead cannot be made before the customer exists.
    let (status, _) = call(
        &app,
        SALES,
        Method::POST,
        &format!("/api/v1/tickets/{tid}/lead"),
        Some(json!({})),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);

    // Register the industrial customer with a contact detail, and find it again by name.
    let (status, c) = call(
        &app,
        SALES,
        Method::POST,
        "/api/v1/customers",
        Some(json!({
            "code": "C0901",
            "name": TEST_CUSTOMER_NAME,
            "email": "kari@gmail.com",
            "phone": "+47 22 00 00 00",
            "country_code": TEST_CUSTOMER_COUNTRY,
            "language": TEST_CUSTOMER_LANGUAGE,
            "price_zone_id": ZONE_INDUSTRY,
        })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{c}");
    assert_eq!(c["phone"], "+47 22 00 00 00");
    let cid = c["id"].as_str().unwrap().to_string();
    let (_, found) = call(&app, SALES, Method::GET, "/api/v1/customers?q=Nordic", None).await;
    let names: Vec<&str> = found["items"]
        .as_array()
        .unwrap()
        .iter()
        .map(|i| i["name"].as_str().unwrap())
        .collect();
    assert_eq!(names, [TEST_CUSTOMER_NAME], "{found}");

    // Link the ticket to the customer.
    let mut input = t["ticket"].clone();
    input["customer_id"] = json!(cid);
    let (status, u) = call(
        &app,
        SALES,
        Method::PUT,
        &format!("/api/v1/tickets/{tid}"),
        Some(input),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{u}");
    let t = get_ticket(&app, SUPPORT, &tid).await;
    assert_eq!(t["ticket"]["customer_id"], cid.as_str());
    assert_eq!(t["ticket"]["customer_name"], TEST_CUSTOMER_NAME);

    // Convert the lead into an opportunity for that customer.
    let (status, t) = call(
        &app,
        SALES,
        Method::POST,
        &format!("/api/v1/tickets/{tid}/lead"),
        Some(json!({ "title": "Process unit with pumps", "expected_value": "12000" })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{t}");
    let number = t["ticket"]["opportunity_number"].as_str().unwrap();
    assert!(number.starts_with('O') && number.len() > 1, "{number}");
    let oid = t["ticket"]["opportunity_id"].as_str().unwrap();
    let (status, o) = call(
        &app,
        SALES,
        Method::GET,
        &format!("/api/v1/opportunities/{oid}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{o}");
    assert_eq!(o["number"], number);
    assert_eq!(o["customer_id"], cid.as_str());
    assert_eq!(o["title"], "Process unit with pumps");

    // A ticket is converted only once.
    let (status, _) = call(
        &app,
        SALES,
        Method::POST,
        &format!("/api/v1/tickets/{tid}/lead"),
        Some(json!({})),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);
}
