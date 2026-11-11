//! Customer closures: periods in which a customer cannot receive parcels or shipments.

mod common;

use axum::http::{Method, StatusCode};
use chrono::NaiveDate;
use common::{call, setup};
use serde_json::{json, Value};
use sqlx::PgPool;

const ALPHA: &str = "00000000-0000-7000-8000-000000000401";
const BETA: &str = "00000000-0000-7000-8000-000000000402";
const SALES: &str = "sales@example.com";
const SUPPORT: &str = "support@example.com";
const VIEWER: &str = "viewer@example.com";
const PRODUCTION: &str = "production@example.com";

async fn get(app: &axum::Router, user: &str, uri: &str) -> (StatusCode, Value) {
    call(app, user, Method::GET, uri, None).await
}

async fn post(app: &axum::Router, user: &str, uri: &str, body: Value) -> (StatusCode, Value) {
    call(app, user, Method::POST, uri, Some(body)).await
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn staff_record_and_maintain_closures(db: PgPool) {
    let app = setup(db).await;
    let uri = format!("/api/v1/customers/{ALPHA}/closures");
    let body = json!({
        "starts_on": "2099-12-24", "ends_on": "2100-01-02",
        "kind": "collective_leave", "notes": "  Christmas break  "
    });

    // Reading is open to customer readers, writing to those who maintain deliveries.
    assert_eq!(
        post(&app, VIEWER, &uri, body.clone()).await.0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        post(&app, PRODUCTION, &uri, body.clone()).await.0,
        StatusCode::FORBIDDEN
    );
    let (status, c) = post(&app, SALES, &uri, body.clone()).await;
    assert_eq!(status, StatusCode::CREATED, "{c}");
    let first = c["id"].as_str().unwrap().to_string();
    assert_eq!(c["starts_on"], "2099-12-24");
    assert_eq!(c["ends_on"], "2100-01-02");
    assert_eq!(c["kind"], "collective_leave");
    assert_eq!(c["notes"], "Christmas break");
    assert_eq!(c["source"], "staff");
    assert_eq!(c["customer_name"], "Alpha Installaties NV");

    // A single day needs no end date.
    let (status, one) = post(
        &app,
        SUPPORT,
        &uri,
        json!({ "starts_on": "2099-06-01", "kind": "inventory" }),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{one}");
    assert_eq!(one["ends_on"], "2099-06-01");

    let (status, list) = get(&app, VIEWER, &uri).await;
    assert_eq!(status, StatusCode::OK);
    let days: Vec<_> = list
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c["starts_on"].clone())
        .collect();
    assert_eq!(
        days,
        vec![json!("2099-06-01"), json!("2099-12-24")],
        "ordered by start"
    );

    // Change and remove it.
    let id = one["id"].as_str().unwrap();
    let (status, c) = call(
        &app,
        SALES,
        Method::PUT,
        &format!("{uri}/{id}"),
        Some(json!({ "starts_on": "2099-06-01", "ends_on": "2099-06-03", "kind": "works" })),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{c}");
    assert_eq!(c["ends_on"], "2099-06-03");
    assert_eq!(c["kind"], "works");
    let (status, _) = call(&app, VIEWER, Method::DELETE, &format!("{uri}/{id}"), None).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    let (status, _) = call(&app, SALES, Method::DELETE, &format!("{uri}/{id}"), None).await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    let (status, _) = call(&app, SALES, Method::DELETE, &format!("{uri}/{id}"), None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    // A closure of another customer, still there, cannot be changed or removed through this
    // customer's URL.
    let beta = format!("/api/v1/customers/{BETA}/closures/{first}");
    let (status, _) = call(
        &app,
        SALES,
        Method::PUT,
        &beta,
        Some(json!({ "starts_on": "2099-06-01" })),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    let (status, _) = call(&app, SALES, Method::DELETE, &beta, None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    let (_, list) = get(&app, SALES, &uri).await;
    let list = list.as_array().unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0]["id"], first.as_str());
    assert_eq!(list[0]["starts_on"], "2099-12-24");
    assert_eq!(list[0]["notes"], "Christmas break");
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn closures_are_validated(db: PgPool) {
    let app = setup(db).await;
    let uri = format!("/api/v1/customers/{ALPHA}/closures");
    for body in [
        json!({ "starts_on": "2099-02-02", "ends_on": "2099-02-01" }),
        json!({ "starts_on": "2099-02-02", "kind": "party" }),
        json!({ "starts_on": "2099-01-01", "ends_on": "2101-01-01" }),
    ] {
        let (status, err) = post(&app, SALES, &uri, body.clone()).await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body} {err}");
    }
    let unknown = "00000000-0000-7000-8000-0000000009ff";
    let (status, _) = post(
        &app,
        SALES,
        &format!("/api/v1/customers/{unknown}/closures"),
        json!({ "starts_on": "2099-02-02" }),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn lists_show_what_is_still_relevant(db: PgPool) {
    let app = setup(db).await;
    for (customer, body) in [
        (
            ALPHA,
            json!({ "starts_on": "2020-01-01", "ends_on": "2020-01-05" }),
        ),
        (
            ALPHA,
            json!({ "starts_on": "2099-03-01", "ends_on": "2099-03-05" }),
        ),
        (BETA, json!({ "starts_on": "2099-02-01" })),
    ] {
        let uri = format!("/api/v1/customers/{customer}/closures");
        assert_eq!(post(&app, SALES, &uri, body).await.0, StatusCode::CREATED);
    }
    // Past closures drop off by default but can be asked for.
    let (_, list) = get(&app, SALES, &format!("/api/v1/customers/{ALPHA}/closures")).await;
    assert_eq!(list.as_array().unwrap().len(), 1);
    let (_, list) = get(
        &app,
        SALES,
        &format!("/api/v1/customers/{ALPHA}/closures?from=2019-01-01"),
    )
    .await;
    assert_eq!(list.as_array().unwrap().len(), 2);

    // All customers, with names, for delivery planning; bounded by a period.
    let (status, all) = get(&app, SALES, "/api/v1/closures").await;
    assert_eq!(status, StatusCode::OK);
    let names: Vec<_> = all
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c["customer_name"].clone())
        .collect();
    assert_eq!(
        names,
        vec![json!("Bêta Industrie SA"), json!("Alpha Installaties NV")]
    );
    let (_, all) = get(
        &app,
        SALES,
        "/api/v1/closures?from=2099-03-01&to=2099-03-31",
    )
    .await;
    assert_eq!(all.as_array().unwrap().len(), 1);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn lists_include_closures_that_overlap_the_period(db: PgPool) {
    let app = setup(db).await;
    let uri = format!("/api/v1/customers/{ALPHA}/closures");
    // Period asked for: 10 to 20 June.
    let mut ids = std::collections::HashMap::new();
    for (name, from, to) in [
        ("before", "2099-06-01", "2099-06-09"),
        ("touches_start", "2099-06-05", "2099-06-10"),
        ("crosses_start", "2099-06-08", "2099-06-12"),
        ("inside", "2099-06-14", "2099-06-15"),
        ("encloses", "2099-06-01", "2099-06-30"),
        ("crosses_end", "2099-06-18", "2099-06-25"),
        ("touches_end", "2099-06-20", "2099-06-25"),
        ("after", "2099-06-21", "2099-06-25"),
    ] {
        let (status, c) = post(
            &app,
            SALES,
            &uri,
            json!({ "starts_on": from, "ends_on": to }),
        )
        .await;
        assert_eq!(status, StatusCode::CREATED, "{c}");
        ids.insert(c["id"].as_str().unwrap().to_string(), name);
    }
    let query = "from=2099-06-10&to=2099-06-20";
    for path in [uri.clone(), "/api/v1/closures".to_string()] {
        let (status, list) = get(&app, SALES, &format!("{path}?{query}")).await;
        assert_eq!(status, StatusCode::OK, "{list}");
        let mut names: Vec<_> = list
            .as_array()
            .unwrap()
            .iter()
            .map(|c| ids[c["id"].as_str().unwrap()])
            .collect();
        names.sort();
        assert_eq!(
            names,
            vec![
                "crosses_end",
                "crosses_start",
                "encloses",
                "inside",
                "touches_end",
                "touches_start"
            ],
            "{path}"
        );
    }
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn adjacent_closures_chain_in_the_next_open_day(db: PgPool) {
    let app = setup(db.clone()).await;
    let uri = format!("/api/v1/customers/{ALPHA}/closures");
    // One ends on the 12th, the next starts on the 13th: closed 10 to 15 June without overlap.
    for body in [
        json!({ "starts_on": "2099-06-10", "ends_on": "2099-06-12" }),
        json!({ "starts_on": "2099-06-13", "ends_on": "2099-06-15" }),
    ] {
        assert_eq!(post(&app, SALES, &uri, body).await.0, StatusCode::CREATED);
    }
    let (_, chk) = get(&app, VIEWER, &format!("{uri}/check?date=2099-06-11")).await;
    assert_eq!(chk["next_open_day"], "2099-06-16");
    let mut conn = db.acquire().await.unwrap();
    let next = tradocraft_crm::closures::next_open_day(
        &mut conn,
        ALPHA.parse().unwrap(),
        NaiveDate::from_ymd_opt(2099, 6, 10).unwrap(),
    )
    .await
    .unwrap();
    assert_eq!(next, NaiveDate::from_ymd_opt(2099, 6, 16).unwrap());
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn delivery_planning_finds_the_next_open_day(db: PgPool) {
    let app = setup(db.clone()).await;
    let uri = format!("/api/v1/customers/{ALPHA}/closures");
    // Two closures that touch: the customer is closed 10 to 15 June.
    for body in [
        json!({ "starts_on": "2099-06-10", "ends_on": "2099-06-12" }),
        json!({ "starts_on": "2099-06-12", "ends_on": "2099-06-15" }),
    ] {
        assert_eq!(post(&app, SALES, &uri, body).await.0, StatusCode::CREATED);
    }
    let (status, chk) = get(&app, VIEWER, &format!("{uri}/check?date=2099-06-11")).await;
    assert_eq!(status, StatusCode::OK, "{chk}");
    assert_eq!(chk["closed"], true);
    assert_eq!(chk["closure"]["kind"], "other");
    assert_eq!(chk["next_open_day"], "2099-06-16");
    let (_, chk) = get(&app, VIEWER, &format!("{uri}/check?date=2099-06-20")).await;
    assert_eq!(
        (chk["closed"].clone(), chk["closure"].clone()),
        (json!(false), Value::Null)
    );
    assert_eq!(chk["next_open_day"], "2099-06-20");
    let (status, _) = get(&app, VIEWER, &format!("{uri}/check?date=nope")).await;
    assert!(status.is_client_error());

    let alpha: uuid::Uuid = ALPHA.parse().unwrap();
    let day = |s: &str| NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap();

    assert!(
        tradocraft_crm::closures::closure_on(&db, alpha, day("2099-06-09"))
            .await
            .unwrap()
            .is_none()
    );
    let c = tradocraft_crm::closures::closure_on(&db, alpha, day("2099-06-12"))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(c.ends_on, day("2099-06-15"), "the one that ends last");

    let mut conn = db.acquire().await.unwrap();
    for (from, open) in [
        ("2099-06-09", "2099-06-09"),
        ("2099-06-10", "2099-06-16"),
        ("2099-06-15", "2099-06-16"),
        ("2099-06-16", "2099-06-16"),
    ] {
        let next = tradocraft_crm::closures::next_open_day(&mut conn, alpha, day(from))
            .await
            .unwrap();
        assert_eq!(next, day(open), "from {from}");
    }
}
