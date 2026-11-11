//! Quotation line remarks (ADR 0037): the production remark (internal, defaulted from the product)
//! and the customer remark (printed), through create, update, revise, order conversion and the
//! PDF, and the remark history at the scopes customer, language and all.

mod common;

use axum::http::{Method, StatusCode};
use common::{call, raw, setup, Who};
use serde_json::{json, Value};
use sqlx::PgPool;

const SALES: &str = "sales@example.com";
const ALPHA: &str = "00000000-0000-7000-8000-000000000401"; // nl
const BETA: &str = "00000000-0000-7000-8000-000000000402"; // fr
const GAMMA: &str = "00000000-0000-7000-8000-000000000403"; // de
const UNIT: &str = "00000000-0000-7000-8000-000000000301";
const PUMP: &str = "00000000-0000-7000-8000-000000000303";

/// Sends a request to the API as the sales user and returns status and body.
async fn api(
    app: &axum::Router,
    method: Method,
    uri: &str,
    body: Option<Value>,
) -> (StatusCode, Value) {
    call(app, SALES, method, &format!("/api/v1{uri}"), body).await
}

/// Creates a quotation for the customer with the given lines.
async fn quote(app: &axum::Router, customer: &str, lines: Value) -> Value {
    let (status, q) = api(
        app,
        Method::POST,
        "/quotations",
        Some(json!({ "customer_id": customer, "lines": lines })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{q}");
    q
}

/// The remark history of a product for the query string.
async fn history(app: &axum::Router, product: &str, query: &str) -> Vec<Value> {
    let (status, h) = api(
        app,
        Method::GET,
        &format!("/products/{product}/remark-history?{query}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{h}");
    h.as_array().unwrap().clone()
}

/// The texts of the history entries.
fn texts(h: &[Value]) -> Vec<&str> {
    h.iter().map(|e| e["text"].as_str().unwrap()).collect()
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn production_remark_defaults_from_the_product(db: PgPool) {
    let app = setup(db).await;
    let (status, p) = api(&app, Method::GET, &format!("/products/{UNIT}"), None).await;
    assert_eq!(status, StatusCode::OK);
    let mut input = json!({
        "code": p["code"], "name": p["name"], "vat_rate": p["vat_rate"], "unit": p["unit"],
        "product_type": p["product_type"], "default_production_remark": "  Weld the frame first  "
    });
    let (status, u) = call(
        &app,
        "admin@example.com",
        Method::PUT,
        &format!("/api/v1/products/{UNIT}"),
        Some(input.clone()),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{u}");
    assert_eq!(u["default_production_remark"], "Weld the frame first");
    input["default_production_remark"] = json!("x".repeat(2001));
    let (status, _) = call(
        &app,
        "admin@example.com",
        Method::PUT,
        &format!("/api/v1/products/{UNIT}"),
        Some(input),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);

    let q = quote(
        &app,
        ALPHA,
        json!([
            { "product_id": UNIT, "quantity": "1" },
            { "product_id": UNIT, "quantity": "1", "production_remark": "Custom note" },
            { "product_id": PUMP, "quantity": "1" },
        ]),
    )
    .await;
    assert_eq!(q["lines"][0]["production_remark"], "Weld the frame first");
    assert_eq!(q["lines"][1]["production_remark"], "Custom note");
    assert_eq!(q["lines"][2]["production_remark"], "");
    assert_eq!(q["lines"][2]["customer_remark"], "");

    let (status, e) = api(
        &app,
        Method::POST,
        "/quotations",
        Some(json!({ "customer_id": ALPHA, "lines": [
            { "product_id": UNIT, "quantity": "1", "customer_remark": "y".repeat(2001) }] })),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{e}");
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn remarks_survive_update_revision_and_order(db: PgPool) {
    let app = setup(db).await;
    let lines = json!([{ "product_id": PUMP, "quantity": "2",
        "production_remark": "Paint RAL 5010", "customer_remark": "Delivered on a pallet" }]);
    let q = quote(&app, ALPHA, lines.clone()).await;
    let id = q["id"].as_str().unwrap();
    assert_eq!(q["lines"][0]["production_remark"], "Paint RAL 5010");
    assert_eq!(q["lines"][0]["customer_remark"], "Delivered on a pallet");

    // PUT replaces the lines: remarks sent again are stored, left out they are cleared.
    let (status, u) = api(
        &app,
        Method::PUT,
        &format!("/quotations/{id}"),
        Some(
            json!({ "customer_id": ALPHA, "lines": [{ "product_id": PUMP, "quantity": "3",
            "production_remark": "Paint RAL 7016", "customer_remark": "Delivered on a pallet" }] }),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{u}");
    assert_eq!(u["lines"][0]["production_remark"], "Paint RAL 7016");
    let (_, got) = api(&app, Method::GET, &format!("/quotations/{id}"), None).await;
    assert_eq!(got["lines"][0]["customer_remark"], "Delivered on a pallet");

    let (status, _) = api(&app, Method::POST, &format!("/quotations/{id}/send"), None).await;
    assert_eq!(status, StatusCode::OK);

    // Revisions keep the remarks, with the agreed prices and with today's prices.
    let mut current = id.to_string();
    for pricing in ["keep", "current"] {
        let (status, r) = api(
            &app,
            Method::POST,
            &format!("/quotations/{current}/revise"),
            Some(json!({ "pricing": pricing })),
        )
        .await;
        assert_eq!(status, StatusCode::CREATED, "{r}");
        assert_eq!(
            r["lines"][0]["production_remark"], "Paint RAL 7016",
            "{pricing}"
        );
        assert_eq!(
            r["lines"][0]["customer_remark"], "Delivered on a pallet",
            "{pricing}"
        );
        current = r["id"].as_str().unwrap().to_string();
        let (status, _) = api(
            &app,
            Method::POST,
            &format!("/quotations/{current}/send"),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK);
    }

    // The order takes the remarks, and so does each order revision.
    let (status, o) = api(
        &app,
        Method::POST,
        &format!("/quotations/{current}/convert"),
        Some(json!({})),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{o}");
    assert_eq!(o["lines"][0]["production_remark"], "Paint RAL 7016");
    assert_eq!(o["lines"][0]["customer_remark"], "Delivered on a pallet");
    let oid = o["order"]["id"].as_str().unwrap();
    let (status, o) = api(&app, Method::POST, &format!("/orders/{oid}/send"), None).await;
    assert_eq!(status, StatusCode::OK, "{o}");
    let (status, o) = api(
        &app,
        Method::POST,
        &format!("/orders/{oid}/revisions"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{o}");
    assert_eq!(o["lines"][0]["customer_remark"], "Delivered on a pallet");
    assert_eq!(o["lines"][0]["production_remark"], "Paint RAL 7016");
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn customer_remark_is_printed_and_production_remark_is_not(db: PgPool) {
    if std::process::Command::new(std::env::var("TYPST_BIN").unwrap_or_else(|_| "typst".into()))
        .arg("--version")
        .output()
        .is_err()
    {
        panic!("typst not installed: set TYPST_BIN");
    }
    let app = setup(db).await;
    let q = quote(
        &app,
        ALPHA,
        json!([{ "product_id": PUMP, "quantity": "1",
            "production_remark": "SECRETPRODUCTIONNOTE", "customer_remark": "VISIBLECUSTOMERNOTE" }]),
    )
    .await;
    let id = q["id"].as_str().unwrap();
    let (status, _, pdf) = raw(
        &app,
        Who::Staff(SALES),
        Method::GET,
        &format!("/api/v1/quotations/{id}/pdf"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(pdf.starts_with(b"%PDF"));
    let dir = std::env::temp_dir().join(format!("remark-pdf-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("q.pdf"), &pdf).unwrap();
    // The text of the PDF; the check fails when pdftotext is missing so it never passes unchecked.
    let out = std::process::Command::new("pdftotext")
        .arg(dir.join("q.pdf"))
        .arg("-")
        .output()
        .expect("pdftotext (poppler-utils) must be installed to check the printed remarks");
    let text = String::from_utf8_lossy(&out.stdout).to_string();
    assert!(text.contains("VISIBLECUSTOMERNOTE"), "{text}");
    assert!(!text.contains("SECRETPRODUCTIONNOTE"), "{text}");
    let _ = std::fs::remove_dir_all(&dir);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn remark_history_has_three_scopes(db: PgPool) {
    let app = setup(db).await;
    // Alpha (nl) and Beta (fr) use different customer remarks on the pump; Gamma (de) too.
    quote(
        &app,
        ALPHA,
        json!([{ "product_id": PUMP, "quantity": "1", "customer_remark": "Alpha note" }]),
    )
    .await;
    quote(
        &app,
        ALPHA,
        json!([{ "product_id": PUMP, "quantity": "1", "customer_remark": "Alpha note",
        "production_remark": "Alpha shop" }]),
    )
    .await;
    quote(
        &app,
        BETA,
        json!([{ "product_id": PUMP, "quantity": "1", "customer_remark": "Beta note" }]),
    )
    .await;
    quote(
        &app,
        GAMMA,
        json!([{ "product_id": PUMP, "quantity": "1", "customer_remark": "Gamma note" }]),
    )
    .await;
    // A quotation line of another product does not count.
    quote(
        &app,
        ALPHA,
        json!([{ "product_id": UNIT, "quantity": "1", "customer_remark": "Unit note" }]),
    )
    .await;

    let all = history(&app, PUMP, "kind=customer&scope=all").await;
    let mut got = texts(&all);
    got.sort();
    assert_eq!(got, ["Alpha note", "Beta note", "Gamma note"]);
    let alpha = all.iter().find(|e| e["text"] == "Alpha note").unwrap();
    assert_eq!(alpha["use_count"], 2);
    assert!(alpha["last_quotation_number"].as_str().unwrap().len() > 3);
    assert!(alpha["last_used"].is_string());

    let cust = history(
        &app,
        PUMP,
        &format!("kind=customer&scope=customer&customer_id={BETA}"),
    )
    .await;
    assert_eq!(texts(&cust), ["Beta note"]);
    let lang = history(&app, PUMP, "kind=customer&scope=language&language=nl").await;
    assert_eq!(texts(&lang), ["Alpha note"]);
    let lang = history(&app, PUMP, "kind=customer&scope=language&language=de").await;
    assert_eq!(texts(&lang), ["Gamma note"]);
    // Empty texts are left out; production remarks are a separate list.
    let prod = history(&app, PUMP, "kind=production&scope=all").await;
    assert_eq!(texts(&prod), ["Alpha shop"]);

    // Missing scope parameters and unknown values are refused.
    for q in [
        "kind=customer&scope=customer",
        "kind=customer&scope=language",
        "kind=other&scope=all",
        "kind=customer&scope=nowhere",
    ] {
        let (status, _) = api(
            &app,
            Method::GET,
            &format!("/products/{PUMP}/remark-history?{q}"),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{q}");
    }
}
