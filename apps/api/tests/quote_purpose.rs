//! Purpose (installation | stock) and delivery country of quotations and orders.

mod common;

use axum::http::{Method, StatusCode};
use common::{call, setup};
use serde_json::{json, Value};
use sqlx::PgPool;

const ALPHA: &str = "00000000-0000-7000-8000-000000000401"; // BE
const GAMMA: &str = "00000000-0000-7000-8000-000000000403"; // DE
const UNIT_100: &str = "00000000-0000-7000-8000-000000000301";
const SALES: &str = "sales@example.com";

async fn staff(
    app: &axum::Router,
    method: Method,
    uri: &str,
    body: Option<Value>,
) -> (StatusCode, Value) {
    call(app, SALES, method, &format!("/api/v1{uri}"), body).await
}

fn quote_body(customer: &str, extra: Value) -> Value {
    let mut b =
        json!({ "customer_id": customer, "lines": [{ "product_id": UNIT_100, "quantity": "1" }] });
    for (k, v) in extra.as_object().unwrap() {
        b[k] = v.clone();
    }
    b
}

async fn create(app: &axum::Router, customer: &str, extra: Value) -> Value {
    let (status, q) = staff(
        app,
        Method::POST,
        "/quotations",
        Some(quote_body(customer, extra)),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{q}");
    q
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn purpose_defaults_to_installation_and_accepts_stock(db: PgPool) {
    let app = setup(db).await;
    let q = create(&app, ALPHA, json!({})).await;
    assert_eq!(q["purpose"], "installation");
    let q = create(&app, ALPHA, json!({ "purpose": "stock" })).await;
    assert_eq!(q["purpose"], "stock");
    let (_, page) = staff(&app, Method::GET, "/quotations", None).await;
    let purposes: Vec<&str> = page["items"]
        .as_array()
        .unwrap()
        .iter()
        .map(|i| i["purpose"].as_str().unwrap())
        .collect();
    assert!(purposes.contains(&"stock") && purposes.contains(&"installation"));
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn invalid_purpose_or_country_is_rejected(db: PgPool) {
    let app = setup(db).await;
    for extra in [
        json!({ "purpose": "demo" }),
        json!({ "purpose": "" }),
        json!({ "delivery_country_code": "BEL" }),
        json!({ "delivery_country_code": "B1" }),
    ] {
        let (status, e) = staff(
            &app,
            Method::POST,
            "/quotations",
            Some(quote_body(ALPHA, extra.clone())),
        )
        .await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{extra}: {e}");
    }
    let q = create(&app, ALPHA, json!({})).await;
    let (status, _) = staff(
        &app,
        Method::PUT,
        &format!("/quotations/{}", q["id"].as_str().unwrap()),
        Some(quote_body(ALPHA, json!({ "purpose": "demo" }))),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn delivery_country_defaults_to_the_customer_country_and_is_uppercased(db: PgPool) {
    let app = setup(db).await;
    let q = create(&app, GAMMA, json!({})).await;
    assert_eq!(q["delivery_country_code"], "DE");
    let q = create(&app, GAMMA, json!({ "delivery_country_code": "nl" })).await;
    assert_eq!(q["delivery_country_code"], "NL");
    // Stock without a country has none.
    let q = create(&app, GAMMA, json!({ "purpose": "stock" })).await;
    assert!(q["delivery_country_code"].is_null());

    // An update keeps the country when omitted, and changes it when given.
    let id = q["id"].as_str().unwrap();
    let (_, u) = staff(
        &app,
        Method::PUT,
        &format!("/quotations/{id}"),
        Some(quote_body(GAMMA, json!({ "delivery_country_code": "fr" }))),
    )
    .await;
    assert_eq!(u["delivery_country_code"], "FR");
    let (_, u) = staff(
        &app,
        Method::PUT,
        &format!("/quotations/{id}"),
        Some(quote_body(GAMMA, json!({}))),
    )
    .await;
    assert_eq!(u["delivery_country_code"], "FR");
    assert_eq!(u["purpose"], "stock", "purpose is unchanged when omitted");
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn conversion_copies_both_and_purpose_is_locked_after_confirmation(db: PgPool) {
    let app = setup(db).await;
    let q = create(
        &app,
        GAMMA,
        json!({ "purpose": "stock", "delivery_country_code": "at" }),
    )
    .await;
    let qid = q["id"].as_str().unwrap();
    let (status, _) = staff(&app, Method::POST, &format!("/quotations/{qid}/send"), None).await;
    assert_eq!(status, StatusCode::OK);
    let (status, o) = staff(
        &app,
        Method::POST,
        &format!("/quotations/{qid}/convert"),
        Some(json!({})),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{o}");
    assert_eq!(o["order"]["purpose"], "stock");
    assert_eq!(o["order"]["delivery_country_code"], "AT");
    let oid = o["order"]["id"].as_str().unwrap();

    // Editable on the draft.
    let line = json!([{ "position": 1, "quantity": "1" }]);
    let (status, e) = staff(
        &app,
        Method::PUT,
        &format!("/orders/{oid}"),
        Some(json!({ "purpose": "installation", "delivery_country_code": "be", "lines": line })),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{e}");
    assert_eq!(e["order"]["purpose"], "installation");
    assert_eq!(e["order"]["delivery_country_code"], "BE");
    let (status, _) = staff(
        &app,
        Method::PUT,
        &format!("/orders/{oid}"),
        Some(json!({ "purpose": "bogus", "lines": line })),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);

    let (status, e) = staff(&app, Method::POST, &format!("/orders/{oid}/send"), None).await;
    assert_eq!(status, StatusCode::OK, "{e}");
    let (status, e) = staff(
        &app,
        Method::POST,
        &format!("/orders/{oid}/confirm"),
        Some(json!({})),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{e}");
    assert_eq!(e["order"]["status"], "confirmed");

    let (status, e) = staff(
        &app,
        Method::PUT,
        &format!("/orders/{oid}"),
        Some(json!({ "purpose": "stock", "lines": line })),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT, "{e}");
    let (_, o) = staff(&app, Method::GET, &format!("/orders/{oid}"), None).await;
    assert_eq!(o["order"]["purpose"], "installation");
}
