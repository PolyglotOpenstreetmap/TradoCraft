//! User-defined reports (ADR 0042): run, group, join, scope, share and export.

mod common;

use axum::http::{Method, StatusCode};
use common::{call, raw, setup, Who};
use serde_json::{json, Value};
use sqlx::PgPool;

const COMPANY: &str = "00000000-0000-7000-8000-000000000001";
const OTHER_COMPANY: &str = "00000000-0000-7000-8000-0000000000b1";
const ADMIN_ID: &str = "00000000-0000-7000-8000-000000000101";
const ALPHA: &str = "00000000-0000-7000-8000-000000000401";
const BETA: &str = "00000000-0000-7000-8000-000000000402";
const ADMIN: &str = "admin@example.com";
const SALES: &str = "sales@example.com";
const SUPPORT: &str = "support@example.com";
const VIEWER: &str = "viewer@example.com";
const INTEGRATION: &str = "integration@example.com";
const OTHER: &str = "other@example.com";

async fn api(
    app: &axum::Router,
    user: &str,
    method: Method,
    uri: &str,
    body: Option<Value>,
) -> (StatusCode, Value) {
    call(app, user, method, &format!("/api/v1{uri}"), body).await
}
async fn post(app: &axum::Router, user: &str, uri: &str, body: Value) -> (StatusCode, Value) {
    api(app, user, Method::POST, uri, Some(body)).await
}
async fn put(app: &axum::Router, user: &str, uri: &str, body: Value) -> (StatusCode, Value) {
    api(app, user, Method::PUT, uri, Some(body)).await
}
async fn get(app: &axum::Router, user: &str, uri: &str) -> (StatusCode, Value) {
    api(app, user, Method::GET, uri, None).await
}

/// Company data: four quotations of ours, two orders, and one quotation of another company for
/// the same customer that must never show up.
async fn seed(db: &PgPool) {
    sqlx::query("INSERT INTO core.companies (id, name) VALUES ($1::uuid, 'Other BV')")
        .bind(OTHER_COMPANY)
        .execute(db)
        .await
        .unwrap();
    sqlx::query("INSERT INTO core.users (id, email, display_name) VALUES (gen_random_uuid(), 'other@example.com', 'Other Admin')")
        .execute(db)
        .await
        .unwrap();
    sqlx::query("INSERT INTO core.user_companies (user_id, company_id, roles, is_default) SELECT id, $1::uuid, '{admin}', true FROM core.users WHERE email = 'other@example.com'")
        .bind(OTHER_COMPANY)
        .execute(db)
        .await
        .unwrap();

    // (company, number, status, customer, customer name, date, total)
    let quotations = [
        (
            COMPANY,
            "Q1",
            "sent",
            ALPHA,
            "Alpha Installaties NV",
            "2027-01-10",
            "100.00",
        ),
        (
            COMPANY,
            "Q2",
            "accepted",
            ALPHA,
            "Alpha Installaties NV",
            "2027-01-20",
            "300.00",
        ),
        (
            COMPANY,
            "Q3",
            "draft",
            ALPHA,
            "Alpha Installaties NV",
            "2027-02-05",
            "50.00",
        ),
        (
            COMPANY,
            "Q4",
            "accepted",
            BETA,
            "Bêta Industrie SA",
            "2027-02-10",
            "200.00",
        ),
        (
            OTHER_COMPANY,
            "Q9",
            "accepted",
            ALPHA,
            "Alpha Installaties NV",
            "2027-01-15",
            "999.00",
        ),
    ];
    for (company, number, status, customer, name, date, total) in quotations {
        sqlx::query(
            "INSERT INTO sales.quotations (id, company_id, number, status, customer_id, customer_name,
                customer_street, customer_postal_code, customer_city, customer_country_code, customer_language,
                price_zone_code, customer_discount_percent, currency, quote_date, price_date, valid_until,
                subtotal, vat_total, total, created_by)
             VALUES (gen_random_uuid(), $1::uuid, $2, $3, $4::uuid, $5, 'Street 1', '9000', 'Gent', 'BE', 'nl',
                'BE', 0, 'EUR', $6::date, $6::date, $6::date + 30, $7::numeric, 0, $7::numeric, $8::uuid)",
        )
        .bind(company)
        .bind(number)
        .bind(status)
        .bind(customer)
        .bind(name)
        .bind(date)
        .bind(total)
        .bind(ADMIN_ID)
        .execute(db)
        .await
        .unwrap();
    }
    for (number, quotation, customer, name, date, total) in [
        (
            "O2",
            "Q2",
            ALPHA,
            "Alpha Installaties NV",
            "2027-01-21",
            "300.00",
        ),
        (
            "O4",
            "Q4",
            BETA,
            "Bêta Industrie SA",
            "2027-02-11",
            "200.00",
        ),
    ] {
        sqlx::query(
            "INSERT INTO sales.orders (id, company_id, number, quotation_id, customer_id, customer_name,
                order_date, currency, subtotal, vat_total, total, salesperson_id, created_by)
             SELECT gen_random_uuid(), $1::uuid, $2, q.id, $4::uuid, $5, $6::date, 'EUR', $7::numeric, 0,
                    $7::numeric, $8::uuid, $8::uuid
             FROM sales.quotations q WHERE q.company_id = $1::uuid AND q.number = $3",
        )
        .bind(COMPANY)
        .bind(number)
        .bind(quotation)
        .bind(customer)
        .bind(name)
        .bind(date)
        .bind(total)
        .bind(ADMIN_ID)
        .execute(db)
        .await
        .unwrap();
    }
}

fn col(v: &Value, i: usize) -> Vec<Value> {
    v["rows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r[i].clone())
        .collect()
}

fn strings(v: Vec<Value>) -> Vec<String> {
    v.iter().map(|x| x.as_str().unwrap().to_string()).collect()
}

fn nums(v: Vec<Value>) -> Vec<f64> {
    v.iter().map(|x| x.as_f64().unwrap()).collect()
}

async fn run(app: &axum::Router, user: &str, definition: Value) -> (StatusCode, Value) {
    post(
        app,
        user,
        "/custom-reports/run",
        json!({ "definition": definition }),
    )
    .await
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn plain_report_filters_sorts_pages_and_stays_in_the_company(db: PgPool) {
    let app = setup(db.clone()).await;
    seed(&db).await;
    let def = json!({
        "object": "quotation",
        "columns": [{"field": "quotation.number"}, {"field": "quotation.total"}],
        "filters": [{"field": "quotation.total", "op": "gte", "value": 100}],
        "sort": [{"column": "quotation.total", "dir": "desc"}]
    });
    let (status, r) = run(&app, SALES, def.clone()).await;
    assert_eq!(status, StatusCode::OK, "{r}");
    // Q9 (999, other company) is not there.
    assert_eq!(strings(col(&r, 0)), ["Q2", "Q4", "Q1"]);
    assert_eq!(nums(col(&r, 1)), [300.0, 200.0, 100.0]);
    assert_eq!(r["total"], 3);
    assert_eq!(r["columns"][0]["label"], "Number");
    assert_eq!(r["columns"][1]["type"], "number");
    assert!(r["totals"].is_null());

    let (_, page) = post(
        &app,
        SALES,
        "/custom-reports/run",
        json!({ "definition": def, "limit": 2, "offset": 2 }),
    )
    .await;
    assert_eq!(strings(col(&page, 0)), ["Q1"]);
    assert_eq!(page["total"], 3);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn filters_combine_with_and_or_and_handle_dates_lists_and_nulls(db: PgPool) {
    let app = setup(db.clone()).await;
    seed(&db).await;
    let rows = |filters: Value, logic: &'static str| {
        let app = app.clone();
        async move {
            let (s, r) = run(
                &app,
                SALES,
                json!({"object": "quotation", "columns": [{"field": "quotation.number"}],
                       "filters": filters, "filter_logic": logic,
                       "sort": [{"column": "quotation.number"}]}),
            )
            .await;
            assert_eq!(s, StatusCode::OK, "{r}");
            strings(col(&r, 0))
        }
    };
    assert_eq!(
        rows(
            json!([{"field": "quotation.status", "op": "eq", "value": "accepted"},
                    {"field": "quotation.quote_date", "op": "lt", "value": "2027-02-01"}]),
            "and"
        )
        .await,
        ["Q2"]
    );
    assert_eq!(
        rows(
            json!([{"field": "quotation.status", "op": "eq", "value": "draft"},
                    {"field": "quotation.total", "op": "gt", "value": 250}]),
            "or"
        )
        .await,
        ["Q2", "Q3"]
    );
    assert_eq!(
        rows(
            json!([{"field": "quotation.status", "op": "in", "value": ["sent", "draft"]}]),
            "and"
        )
        .await,
        ["Q1", "Q3"]
    );
    assert_eq!(
        rows(
            json!([{"field": "quotation.customer_name", "op": "contains", "value": "bêta"}]),
            "and"
        )
        .await,
        ["Q4"]
    );
    assert_eq!(
        rows(
            json!([{"field": "quotation.number", "op": "starts_with", "value": "Q"}]),
            "and"
        )
        .await
        .len(),
        4
    );
    // `%` is literal text, not a wildcard.
    assert!(rows(
        json!([{"field": "quotation.number", "op": "contains", "value": "%"}]),
        "and"
    )
    .await
    .is_empty());
    // Nothing was sent, so all four are not sent yet.
    assert_eq!(
        rows(
            json!([{"field": "quotation.sent_at", "op": "is_null"}]),
            "and"
        )
        .await
        .len(),
        4
    );
    assert!(rows(
        json!([{"field": "quotation.sent_at", "op": "not_null"}]),
        "and"
    )
    .await
    .is_empty());
    // A date-time field is compared by day.
    assert_eq!(
        rows(
            json!([{"field": "quotation.created_at", "op": "gte", "value": "2000-01-01"}]),
            "and"
        )
        .await
        .len(),
        4
    );
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn grouped_report_has_group_rows_and_totals(db: PgPool) {
    let app = setup(db.clone()).await;
    seed(&db).await;
    let (status, r) = run(
        &app,
        SALES,
        json!({
            "object": "quotation",
            "columns": [{"field": "quotation.total", "summary": "sum"},
                        {"field": "quotation.records", "summary": "count"},
                        {"field": "quotation.total", "summary": "avg"}],
            "group_by": [{"field": "quotation.status"}],
            "chart": {"kind": "pie", "x": "quotation.status", "y": "quotation.total:sum"}
        }),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{r}");
    let keys: Vec<&str> = r["columns"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c["key"].as_str().unwrap())
        .collect();
    assert_eq!(
        keys,
        [
            "quotation.status",
            "quotation.total:sum",
            "quotation.records:count",
            "quotation.total:avg"
        ]
    );
    assert_eq!(r["columns"][0]["kind"], "group");
    assert_eq!(r["columns"][1]["kind"], "summary");
    assert_eq!(strings(col(&r, 0)), ["accepted", "draft", "sent"]);
    assert_eq!(nums(col(&r, 1)), [500.0, 50.0, 100.0]);
    assert_eq!(nums(col(&r, 2)), [2.0, 1.0, 1.0]);
    assert_eq!(nums(col(&r, 3)), [250.0, 50.0, 100.0]);
    assert_eq!(r["total"], 3);
    let totals = r["totals"].as_array().unwrap();
    assert!(totals[0].is_null());
    assert_eq!(totals[1].as_f64(), Some(650.0));
    assert_eq!(totals[2].as_f64(), Some(4.0));
    assert_eq!(r["chart"]["kind"], "pie");
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn dates_group_by_month_and_two_levels_nest(db: PgPool) {
    let app = setup(db.clone()).await;
    seed(&db).await;
    let (status, r) = run(
        &app,
        SALES,
        json!({
            "object": "quotation",
            "columns": [{"field": "quotation.total", "summary": "sum"}],
            "group_by": [{"field": "quotation.quote_date", "bucket": "month"}]
        }),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{r}");
    assert_eq!(strings(col(&r, 0)), ["2027-01-01", "2027-02-01"]);
    assert_eq!(nums(col(&r, 1)), [400.0, 250.0]);
    assert_eq!(r["columns"][0]["type"], "date");

    let (_, r) = run(
        &app,
        SALES,
        json!({
            "object": "quotation",
            "columns": [{"field": "quotation.total", "summary": "sum"}],
            "group_by": [{"field": "quotation.quote_date", "bucket": "year"},
                         {"field": "quotation.status"}],
            "sort": [{"column": "quotation.total:sum", "dir": "desc"}]
        }),
    )
    .await;
    assert_eq!(strings(col(&r, 1)), ["accepted", "sent", "draft"]);
    assert_eq!(strings(col(&r, 0)), ["2027-01-01"; 3]);

    // Summaries without a grouping give one row.
    let (_, r) = run(
        &app,
        SALES,
        json!({"object": "quotation",
               "columns": [{"field": "quotation.total", "summary": "max"},
                           {"field": "quotation.quote_date", "summary": "min"}]}),
    )
    .await;
    assert_eq!(r["total"], 1);
    assert_eq!(r["rows"][0][0].as_f64(), Some(300.0));
    assert_eq!(r["rows"][0][1], "2027-01-10");
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn joins_combine_objects_and_stay_in_the_company(db: PgPool) {
    let app = setup(db.clone()).await;
    seed(&db).await;

    // Orders with their customer and the quotation they came from.
    let (status, r) = run(
        &app,
        SALES,
        json!({"object": "order", "joins": ["customer", "quotation"],
               "columns": [{"field": "order.number"}, {"field": "customer.name"},
                           {"field": "customer.city"}, {"field": "quotation.status"}],
               "sort": [{"column": "order.number"}]}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{r}");
    assert_eq!(strings(col(&r, 0)), ["O2", "O4"]);
    assert_eq!(
        strings(col(&r, 1)),
        ["Alpha Installaties NV", "Bêta Industrie SA"]
    );
    assert_eq!(strings(col(&r, 3)), ["accepted", "accepted"]);
    assert_eq!(r["columns"][1]["label"], "Customers: Name");

    // Customers with the quotations of this company only: Alpha has 450, not 450 + 999.
    let (status, r) = run(
        &app,
        SALES,
        json!({"object": "customer", "joins": ["quotation"],
               "columns": [{"field": "quotation.total", "summary": "sum"},
                           {"field": "quotation.records", "summary": "count"}],
               "group_by": [{"field": "customer.name"}]}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{r}");
    let names = strings(col(&r, 0));
    let sums = col(&r, 1);
    let counts = nums(col(&r, 2));
    let at = |n: &str| names.iter().position(|x| x == n).unwrap();
    assert_eq!(sums[at("Alpha Installaties NV")].as_f64(), Some(450.0));
    assert_eq!(counts[at("Alpha Installaties NV")], 3.0);
    assert_eq!(sums[at("Bêta Industrie SA")].as_f64(), Some(200.0));
    // A customer without quotations still shows, with nothing to sum.
    assert!(sums[at("Gamma Trading GmbH")].is_null());
    assert_eq!(counts[at("Gamma Trading GmbH")], 0.0);

    // The other company sees its own quotation only, for the same shared customer.
    let (_, r) = run(
        &app,
        OTHER,
        json!({"object": "quotation", "columns": [{"field": "quotation.number"}]}),
    )
    .await;
    assert_eq!(strings(col(&r, 0)), ["Q9"]);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn customer_terms_are_the_runners_company(db: PgPool) {
    let app = setup(db.clone()).await;
    seed(&db).await;
    let (status, r) = run(
        &app,
        SALES,
        json!({"object": "customer",
               "columns": [{"field": "customer.code"}, {"field": "customer.discount_percent"},
                           {"field": "customer.payment_terms_days"}, {"field": "customer.active"}],
               "filters": [{"field": "customer.code", "op": "eq", "value": "C0001"}]}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{r}");
    assert_eq!(r["rows"][0][1].as_f64(), Some(5.0));
    assert_eq!(r["rows"][0][2].as_f64(), Some(30.0));
    assert_eq!(r["rows"][0][3], true);
    // The other company has no terms for this customer.
    let (_, r) = run(
        &app,
        OTHER,
        json!({"object": "customer",
               "columns": [{"field": "customer.code"}, {"field": "customer.discount_percent"}],
               "filters": [{"field": "customer.code", "op": "eq", "value": "C0001"}]}),
    )
    .await;
    assert!(r["rows"][0][1].is_null());
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn invalid_definitions_are_422_and_hostile_text_is_data(db: PgPool) {
    let app = setup(db.clone()).await;
    seed(&db).await;
    for def in [
        json!({"object": "quotation", "columns": [{"field": "quotation.total; DROP TABLE sales.quotations"}]}),
        json!({"object": "sales.quotations", "columns": [{"field": "quotation.total"}]}),
        json!({"object": "quotation", "columns": []}),
        json!({"object": "quotation", "columns": [{"field": "quotation.total"}, {"field": "quotation.total", "summary": "sum"}]}),
        json!({"object": "quotation", "columns": [{"field": "quotation.number"}], "sort": [{"column": "1 --"}]}),
        json!({"object": "quotation", "columns": [{"field": "customer.name"}]}),
        json!({"object": "quotation", "columns": [{"field": "quotation.number"}],
               "filters": [{"field": "quotation.total", "op": "gte", "value": "x"}]}),
    ] {
        let (status, r) = run(&app, SALES, def.clone()).await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{def} -> {r}");
    }
    // A hostile value is just a value that matches nothing.
    let (status, r) = run(
        &app,
        SALES,
        json!({"object": "quotation", "columns": [{"field": "quotation.number"}],
               "filters": [{"field": "quotation.status", "op": "eq", "value": "x' OR '1'='1"}]}),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(r["total"], 0);
    let n: i64 = sqlx::query_scalar("SELECT count(*) FROM sales.quotations")
        .fetch_one(&db)
        .await
        .unwrap();
    assert_eq!(n, 5);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn running_needs_the_report_permission_and_the_objects_read_permission(db: PgPool) {
    let app = setup(db.clone()).await;
    seed(&db).await;
    let def = json!({"object": "quotation", "columns": [{"field": "quotation.number"}]});

    // No reports right at all.
    let (status, _) = run(&app, INTEGRATION, def.clone()).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(
        get(&app, INTEGRATION, "/custom-reports").await.0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        get(&app, INTEGRATION, "/custom-reports/catalog").await.0,
        StatusCode::FORBIDDEN
    );

    // Viewers run reports but cannot save them.
    assert_eq!(run(&app, VIEWER, def.clone()).await.0, StatusCode::OK);
    let (status, _) = post(
        &app,
        VIEWER,
        "/custom-reports",
        json!({"name": "Mine", "definition": def}),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);

    // Without the sales module a user cannot report on quotations, nor join them in.
    sqlx::query("UPDATE core.user_companies SET disabled_modules = '{sales}' WHERE user_id IN (SELECT id FROM core.users WHERE email = $1)")
        .bind(SUPPORT)
        .execute(&db)
        .await
        .unwrap();
    assert_eq!(
        run(&app, SUPPORT, def.clone()).await.0,
        StatusCode::FORBIDDEN
    );
    let (status, _) = run(
        &app,
        SUPPORT,
        json!({"object": "customer", "joins": ["quotation"],
               "columns": [{"field": "customer.name"}, {"field": "quotation.number"}]}),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    let (status, _) = run(
        &app,
        SUPPORT,
        json!({"object": "customer", "columns": [{"field": "customer.name"}]}),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    // The catalog only offers what the user may read.
    let (_, cat) = get(&app, SUPPORT, "/custom-reports/catalog").await;
    let objects: Vec<&str> = cat["objects"]
        .as_array()
        .unwrap()
        .iter()
        .map(|o| o["name"].as_str().unwrap())
        .collect();
    assert_eq!(objects, ["customer"]);
    assert!(cat["links"].as_array().unwrap().is_empty());
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn catalog_lists_objects_fields_and_links(db: PgPool) {
    let app = setup(db.clone()).await;
    let (status, cat) = get(&app, SALES, "/custom-reports/catalog").await;
    assert_eq!(status, StatusCode::OK);
    let objects: Vec<&str> = cat["objects"]
        .as_array()
        .unwrap()
        .iter()
        .map(|o| o["name"].as_str().unwrap())
        .collect();
    assert_eq!(objects, ["customer", "quotation", "order"]);
    let q = &cat["objects"][1];
    assert!(q["fields"]
        .as_array()
        .unwrap()
        .iter()
        .any(|f| f["name"] == "quotation.total" && f["type"] == "number"));
    assert_eq!(cat["links"].as_array().unwrap().len(), 3);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn reports_are_private_until_shared_and_only_owners_change_them(db: PgPool) {
    let app = setup(db.clone()).await;
    seed(&db).await;
    let def = json!({"object": "quotation", "columns": [{"field": "quotation.number"}]});

    let (status, created) = post(
        &app,
        SALES,
        "/custom-reports",
        json!({"name": "  Open quotes ", "description": "d", "definition": def}),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{created}");
    let id = created["id"].as_str().unwrap().to_string();
    assert_eq!(created["name"], "Open quotes");
    assert_eq!(created["shared"], false);
    assert_eq!(created["mine"], true);
    assert_eq!(created["owner_name"], "Sales User");
    assert_eq!(created["object"], "quotation");

    // Private: invisible to colleagues and to other companies.
    for who in [SUPPORT, ADMIN, OTHER] {
        assert_eq!(
            get(&app, who, &format!("/custom-reports/{id}")).await.0,
            StatusCode::NOT_FOUND,
            "{who}"
        );
        assert_eq!(
            get(&app, who, &format!("/custom-reports/{id}/run")).await.0,
            StatusCode::NOT_FOUND,
            "{who}"
        );
        assert_eq!(
            get(&app, who, &format!("/custom-reports/{id}/export"))
                .await
                .0,
            StatusCode::NOT_FOUND,
            "{who}"
        );
        let (_, list) = get(&app, who, "/custom-reports").await;
        assert!(list.as_array().unwrap().is_empty(), "{who}");
    }
    let (_, mine) = get(&app, SALES, "/custom-reports").await;
    assert_eq!(mine.as_array().unwrap().len(), 1);

    // Shared with the company.
    let (status, shared) = put(
        &app,
        SALES,
        &format!("/custom-reports/{id}"),
        json!({"name": "Open quotes", "definition": def, "shared": true}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{shared}");
    assert_eq!(shared["shared"], true);

    let (status, seen) = get(&app, SUPPORT, &format!("/custom-reports/{id}")).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(seen["mine"], false);
    let (status, r) = get(&app, SUPPORT, &format!("/custom-reports/{id}/run")).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(r["total"], 4);
    assert_eq!(
        get(&app, VIEWER, &format!("/custom-reports/{id}/run"))
            .await
            .0,
        StatusCode::OK
    );
    // Still not the other company's.
    assert_eq!(
        get(&app, OTHER, &format!("/custom-reports/{id}")).await.0,
        StatusCode::NOT_FOUND
    );

    // A colleague can run it but not change or delete it.
    let (status, _) = put(
        &app,
        SUPPORT,
        &format!("/custom-reports/{id}"),
        json!({"name": "Hijacked", "definition": def}),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    let (status, _) = api(
        &app,
        SUPPORT,
        Method::DELETE,
        &format!("/custom-reports/{id}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);

    // An admin can.
    let (status, renamed) = put(
        &app,
        ADMIN,
        &format!("/custom-reports/{id}"),
        json!({"name": "Renamed", "definition": def, "shared": true}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{renamed}");
    assert_eq!(renamed["name"], "Renamed");
    assert_eq!(renamed["owner_name"], "Sales User");

    // The owner deletes it.
    let (status, _) = api(
        &app,
        SALES,
        Method::DELETE,
        &format!("/custom-reports/{id}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    assert_eq!(
        get(&app, SALES, &format!("/custom-reports/{id}")).await.0,
        StatusCode::NOT_FOUND
    );

    // Everything was written to the audit trail.
    let actions: Vec<String> = sqlx::query_scalar(
        "SELECT action FROM audit.events WHERE entity = 'report' AND entity_id = $1::uuid ORDER BY id",
    )
    .bind(&id)
    .fetch_all(&db)
    .await
    .unwrap();
    assert_eq!(actions, ["created", "updated", "updated", "deleted"]);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn saving_checks_name_definition_and_the_authors_rights(db: PgPool) {
    let app = setup(db.clone()).await;
    let def = json!({"object": "quotation", "columns": [{"field": "quotation.number"}]});
    for body in [
        json!({"name": "   ", "definition": def}),
        json!({"name": "x".repeat(121), "definition": def}),
        json!({"name": "ok", "definition": {"object": "quotation", "columns": []}}),
        json!({"name": "ok", "definition": {"object": "nope", "columns": [{"field": "nope.x"}]}}),
        json!({"name": "ok", "description": "d".repeat(1001), "definition": def}),
    ] {
        let (status, r) = post(&app, SALES, "/custom-reports", body.clone()).await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body} -> {r}");
    }
    // Updating an unknown report is a 404, not a create.
    let (status, _) = put(
        &app,
        SALES,
        "/custom-reports/00000000-0000-7000-8000-00000000ffff",
        json!({"name": "x", "definition": def}),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    // Nothing was stored by the failures.
    let (_, list) = get(&app, SALES, "/custom-reports").await;
    assert!(list.as_array().unwrap().is_empty());
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn csv_export_has_headings_rows_and_a_totals_line(db: PgPool) {
    let app = setup(db.clone()).await;
    seed(&db).await;
    let (_, created) = post(
        &app,
        SALES,
        "/custom-reports",
        json!({"name": "Total by status / 2027", "definition": {
            "object": "quotation",
            "columns": [{"field": "quotation.total", "summary": "sum", "label": "Amount"}],
            "group_by": [{"field": "quotation.status"}]
        }}),
    )
    .await;
    let id = created["id"].as_str().unwrap();

    let (status, headers, body) = raw(
        &app,
        Who::Staff(SALES),
        Method::GET,
        &format!("/api/v1/custom-reports/{id}/export?format=csv"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(headers["content-type"]
        .to_str()
        .unwrap()
        .starts_with("text/csv"));
    let disposition = headers["content-disposition"].to_str().unwrap();
    assert!(
        disposition.starts_with("attachment; filename=\"report-total-by-status-2027-"),
        "{disposition}"
    );
    assert!(disposition.ends_with(".csv\""));
    let text = String::from_utf8(body).unwrap();
    assert_eq!(
        text,
        "Status,Amount\r\naccepted,500\r\ndraft,50\r\nsent,100\r\n,650\r\n"
    );

    // Same default as the explicit format.
    let (status, _, again) = raw(
        &app,
        Who::Staff(SALES),
        Method::GET,
        &format!("/api/v1/custom-reports/{id}/export"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(String::from_utf8(again).unwrap(), text);

    let (status, _, _) = raw(
        &app,
        Who::Staff(SALES),
        Method::GET,
        &format!("/api/v1/custom-reports/{id}/export?format=docx"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn csv_export_neutralises_spreadsheet_formulas(db: PgPool) {
    let app = setup(db.clone()).await;
    seed(&db).await;
    sqlx::query(
        "UPDATE crm.customers SET name = '=HYPERLINK(\"http://evil\")' WHERE code = 'C0003'",
    )
    .execute(&db)
    .await
    .unwrap();
    let (_, created) = post(
        &app,
        SALES,
        "/custom-reports",
        json!({"name": "Names", "definition": {
            "object": "customer", "columns": [{"field": "customer.name"}],
            "filters": [{"field": "customer.code", "op": "eq", "value": "C0003"}]}}),
    )
    .await;
    let id = created["id"].as_str().unwrap();
    let (_, _, body) = raw(
        &app,
        Who::Staff(SALES),
        Method::GET,
        &format!("/api/v1/custom-reports/{id}/export"),
        None,
    )
    .await;
    let text = String::from_utf8(body).unwrap();
    assert!(text.contains("'=HYPERLINK"), "{text}");
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn pdf_export_renders_the_report(db: PgPool) {
    if std::process::Command::new(std::env::var("TYPST_BIN").unwrap_or_else(|_| "typst".into()))
        .arg("--version")
        .output()
        .is_err()
    {
        eprintln!("typst not installed; skipping PDF test");
        return;
    }
    let app = setup(db.clone()).await;
    seed(&db).await;
    let (_, created) = post(
        &app,
        SALES,
        "/custom-reports",
        json!({"name": "Total by month", "description": "Quotations of 2027", "definition": {
            "object": "quotation",
            "columns": [{"field": "quotation.total", "summary": "sum"}],
            "group_by": [{"field": "quotation.quote_date", "bucket": "month"}],
            "filters": [{"field": "quotation.status", "op": "ne", "value": "draft"}],
            "chart": {"kind": "bar", "x": "quotation.quote_date:month", "y": "quotation.total:sum"}
        }}),
    )
    .await;
    let id = created["id"].as_str().unwrap();
    let (status, headers, body) = raw(
        &app,
        Who::Staff(SALES),
        Method::GET,
        &format!("/api/v1/custom-reports/{id}/export?format=pdf"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{}", String::from_utf8_lossy(&body));
    assert_eq!(headers["content-type"], "application/pdf");
    assert!(headers["content-disposition"]
        .to_str()
        .unwrap()
        .contains("report-total-by-month-"));
    assert!(body.starts_with(b"%PDF-"));

    // A plain report with no rows still renders.
    let (_, empty) = post(
        &app,
        SALES,
        "/custom-reports",
        json!({"name": "Nothing", "definition": {
            "object": "order", "columns": [{"field": "order.number"}],
            "filters": [{"field": "order.status", "op": "eq", "value": "cancelled"}]}}),
    )
    .await;
    let (status, _, body) = raw(
        &app,
        Who::Staff(SALES),
        Method::GET,
        &format!(
            "/api/v1/custom-reports/{}/export?format=pdf",
            empty["id"].as_str().unwrap()
        ),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.starts_with(b"%PDF-"));

    // Not for someone who cannot see the report.
    let (status, _, _) = raw(
        &app,
        Who::Staff(SUPPORT),
        Method::GET,
        &format!("/api/v1/custom-reports/{id}/export?format=pdf"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

async fn set_quote_date(db: &PgPool, number: &str, days_from_today: i32) {
    sqlx::query(
        "UPDATE sales.quotations SET quote_date = CURRENT_DATE + $2::int
         WHERE company_id = $1::uuid AND number = $3",
    )
    .bind(COMPANY)
    .bind(days_from_today)
    .bind(number)
    .execute(db)
    .await
    .unwrap();
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn relative_dates_stay_live_relative_to_today(db: PgPool) {
    let app = setup(db.clone()).await;
    seed(&db).await;
    // Q1 today, Q2 five days ago, Q3 forty days ago, Q4 in three days.
    for (n, d) in [("Q1", 0), ("Q2", -5), ("Q3", -40), ("Q4", 3)] {
        set_quote_date(&db, n, d).await;
    }
    let def = json!({
        "object": "quotation",
        "columns": [{"field": "quotation.number"}],
        "filters": [{"field": "quotation.quote_date", "op": "eq",
                     "value": {"relative": "last_n_days", "n": 30}}],
        "sort": [{"column": "quotation.number"}]
    });
    let (status, created) = post(
        &app,
        SALES,
        "/custom-reports",
        json!({"name": "Recent", "definition": def}),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{created}");
    let id = created["id"].as_str().unwrap().to_string();
    // The token is stored, not the dates it stands for.
    assert_eq!(
        created["definition"]["filters"][0]["value"],
        json!({"relative": "last_n_days", "n": 30})
    );

    let run_uri = format!("/custom-reports/{id}/run");
    let run_saved = || get(&app, SALES, &run_uri);
    let (status, r) = run_saved().await;
    assert_eq!(status, StatusCode::OK, "{r}");
    assert_eq!(strings(col(&r, 0)), ["Q1", "Q2"]);

    // Time passes: Q3 is now within 30 days and Q1 is a month old; the report was not touched.
    set_quote_date(&db, "Q3", -10).await;
    set_quote_date(&db, "Q1", -31).await;
    let (_, r) = run_saved().await;
    assert_eq!(strings(col(&r, 0)), ["Q2", "Q3"]);
    let (_, again) = get(&app, SALES, &format!("/custom-reports/{id}")).await;
    assert_eq!(
        again["definition"]["filters"][0]["value"],
        json!({"relative": "last_n_days", "n": 30})
    );

    // Boundaries: exactly 29 days ago is in (30 days with today), 30 days ago is out.
    set_quote_date(&db, "Q3", -29).await;
    set_quote_date(&db, "Q1", -30).await;
    let (_, r) = run_saved().await;
    assert_eq!(strings(col(&r, 0)), ["Q2", "Q3"]);

    // Other operators and tokens, ad hoc.
    let ad_hoc = |op: &str, value: Value| {
        run(
            &app,
            SALES,
            json!({"object": "quotation", "columns": [{"field": "quotation.number"}],
                   "filters": [{"field": "quotation.quote_date", "op": op, "value": value}],
                   "sort": [{"column": "quotation.number"}]}),
        )
    };
    let (_, r) = ad_hoc("gt", json!({"relative": "today"})).await;
    assert_eq!(strings(col(&r, 0)), ["Q4"]);
    let (_, r) = ad_hoc("eq", json!({"relative": "next_n_days", "n": 3})).await;
    assert!(strings(col(&r, 0)).is_empty());
    let (_, r) = ad_hoc("eq", json!({"relative": "next_n_days", "n": 4})).await;
    assert_eq!(strings(col(&r, 0)), ["Q4"]);
    let (_, r) = ad_hoc("ne", json!({"relative": "last_n_days", "n": 30})).await;
    assert_eq!(strings(col(&r, 0)), ["Q1", "Q4"]);
    let (_, r) = ad_hoc("lt", json!({"relative": "last_year"})).await;
    assert!(strings(col(&r, 0)).is_empty());

    // Date-time fields compare by UTC day too.
    sqlx::query(
        "UPDATE sales.quotations SET sent_at = now() WHERE company_id = $1::uuid AND number = 'Q2'",
    )
    .bind(COMPANY)
    .execute(&db)
    .await
    .unwrap();
    let (status, r) = run(
        &app,
        SALES,
        json!({"object": "quotation", "columns": [{"field": "quotation.number"}],
               "filters": [{"field": "quotation.sent_at", "op": "eq", "value": {"relative": "today"}}]}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{r}");
    assert_eq!(strings(col(&r, 0)), ["Q2"]);

    // Exports resolve it at export time as well.
    let (status, _, csv) = raw(
        &app,
        Who::Staff(SALES),
        Method::GET,
        &format!("/api/v1/custom-reports/{id}/export?format=csv"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let text = String::from_utf8(csv).unwrap();
    assert!(
        text.contains("Q2") && text.contains("Q3") && !text.contains("Q4"),
        "{text}"
    );
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn as_of_keeps_paging_on_one_range(db: PgPool) {
    let app = setup(db.clone()).await;
    seed(&db).await;
    // Q1 today, Q2 five days ago, Q3 forty days ago, Q4 forty-five days ago.
    for (n, d) in [("Q1", 0), ("Q2", -5), ("Q3", -40), ("Q4", -45)] {
        set_quote_date(&db, n, d).await;
    }
    let def = json!({
        "object": "quotation",
        "columns": [{"field": "quotation.number"}],
        "filters": [{"field": "quotation.quote_date", "op": "eq",
                     "value": {"relative": "last_n_days", "n": 30}}],
        "sort": [{"column": "quotation.number"}]
    });
    let today = chrono::Utc::now().date_naive();

    // Absent: today, echoed.
    let (status, r) = run(&app, SALES, def.clone()).await;
    assert_eq!(status, StatusCode::OK, "{r}");
    assert_eq!(r["as_of"], json!(today.to_string()));
    assert_eq!(strings(col(&r, 0)), ["Q1", "Q2"]);

    // An earlier anchor moves the range, and every page of the run keeps it: as of 20 days ago
    // the last 30 days are day -49 to -20, so Q3 and Q4 are in and Q1, Q2 are out.
    let anchor = (today - chrono::Duration::days(20)).to_string();
    let page = |offset: i64| {
        post(
            &app,
            SALES,
            "/custom-reports/run",
            json!({"definition": def, "limit": 1, "offset": offset, "as_of": anchor}),
        )
    };
    let (status, p1) = page(0).await;
    assert_eq!(status, StatusCode::OK, "{p1}");
    let (_, p2) = page(1).await;
    assert_eq!(p1["as_of"], json!(anchor));
    assert_eq!(p2["as_of"], json!(anchor));
    assert_eq!(
        (p1["total"].as_i64(), p2["total"].as_i64()),
        (Some(2), Some(2))
    );
    assert_eq!(strings(col(&p1, 0)), ["Q3"]);
    assert_eq!(strings(col(&p2, 0)), ["Q4"]);

    // Saved report and export take it too.
    let (_, created) = post(
        &app,
        SALES,
        "/custom-reports",
        json!({"name": "Recent", "definition": def}),
    )
    .await;
    let id = created["id"].as_str().unwrap();
    let (status, r) = get(
        &app,
        SALES,
        &format!("/custom-reports/{id}/run?limit=1&offset=1&as_of={anchor}"),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{r}");
    assert_eq!(r["as_of"], json!(anchor));
    assert_eq!(strings(col(&r, 0)), ["Q4"]);
    let (_, r) = get(&app, SALES, &format!("/custom-reports/{id}/run")).await;
    assert_eq!(r["as_of"], json!(today.to_string()));
    let (status, _, csv) = raw(
        &app,
        Who::Staff(SALES),
        Method::GET,
        &format!("/api/v1/custom-reports/{id}/export?format=csv&as_of={anchor}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let text = String::from_utf8(csv).unwrap();
    assert!(
        text.contains("Q3") && text.contains("Q4") && !text.contains("Q1") && !text.contains("Q2"),
        "{text}"
    );
}

/// An app whose report routes see the returned clock's date as today.
async fn app_with_clock(db: PgPool, start: chrono::NaiveDate) -> (axum::Router, Clock) {
    let now = Clock(std::sync::Arc::new(std::sync::Mutex::new(start)));
    let c = now.clone();
    let app = setup(db)
        .await
        .layer(axum::Extension(tradocraft_reports::ReportClock::new(
            move || c.get(),
        )));
    (app, now)
}

/// A per-test settable date (no global state).
#[derive(Clone)]
struct Clock(std::sync::Arc<std::sync::Mutex<chrono::NaiveDate>>);

impl Clock {
    fn get(&self) -> chrono::NaiveDate {
        *self.0.lock().unwrap()
    }
    fn set(&self, d: chrono::NaiveDate) {
        *self.0.lock().unwrap() = d;
    }
}

async fn set_quote_day(db: &PgPool, number: &str, day: chrono::NaiveDate) {
    sqlx::query(
        "UPDATE sales.quotations SET quote_date = $2 WHERE company_id = $1::uuid AND number = $3",
    )
    .bind(COMPANY)
    .bind(day)
    .bind(number)
    .execute(db)
    .await
    .unwrap();
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn paging_across_midnight_keeps_the_first_pages_range(db: PgPool) {
    let d = chrono::Utc::now().date_naive();
    let (app, clock) = app_with_clock(db.clone(), d).await;
    seed(&db).await;
    // On day D the last 3 days are D-2..D: Q1 (D), Q4 (D-1), Q2 (D-2) are in, Q3 (D-3) is out.
    // On day D+1 they are D-1..D+1: Q1 and Q4 only.
    for (n, back) in [("Q1", 0), ("Q4", 1), ("Q2", 2), ("Q3", 3)] {
        set_quote_day(&db, n, d - chrono::Duration::days(back)).await;
    }
    let def = json!({
        "object": "quotation",
        "columns": [{"field": "quotation.number"}],
        "filters": [{"field": "quotation.quote_date", "op": "eq",
                     "value": {"relative": "last_n_days", "n": 3}}],
        "sort": [{"column": "quotation.number"}]
    });
    let run_page = |offset: i64, as_of: Option<String>| {
        let mut body = json!({"definition": def, "limit": 1, "offset": offset});
        if let Some(a) = as_of {
            body["as_of"] = json!(a);
        }
        post(&app, SALES, "/custom-reports/run", body)
    };

    // Page 1 on day D, no as_of: echoed as D.
    let (status, p1) = run_page(0, None).await;
    assert_eq!(status, StatusCode::OK, "{p1}");
    assert_eq!(p1["as_of"], json!(d.to_string()));
    assert_eq!(
        (strings(col(&p1, 0)), p1["total"].as_i64()),
        (vec!["Q1".into()], Some(3))
    );

    // Midnight passes. Pages 2 and 3 carry as_of=D and still see day D's range.
    clock.set(d + chrono::Duration::days(1));
    let (_, p2) = run_page(1, Some(d.to_string())).await;
    let (_, p3) = run_page(2, Some(d.to_string())).await;
    assert_eq!(strings(col(&p2, 0)), ["Q2"]);
    assert_eq!(strings(col(&p3, 0)), ["Q4"]);
    for p in [&p2, &p3] {
        assert_eq!(p["total"].as_i64(), Some(3));
        assert_eq!(p["as_of"], json!(d.to_string()));
    }

    // A fresh run on D+1 uses D+1's range: Q2 dropped out.
    let next = (d + chrono::Duration::days(1)).to_string();
    let (_, fresh) = run_page(0, None).await;
    assert_eq!(fresh["as_of"], json!(next));
    assert_eq!(fresh["total"].as_i64(), Some(2));
    let (_, fresh2) = run_page(1, None).await;
    assert_eq!(strings(col(&fresh2, 0)), ["Q4"]);

    // The saved report and its export honour the same anchor across midnight.
    let (_, created) = post(
        &app,
        SALES,
        "/custom-reports",
        json!({"name": "Recent", "definition": def}),
    )
    .await;
    let id = created["id"].as_str().unwrap();
    let (_, r) = get(
        &app,
        SALES,
        &format!("/custom-reports/{id}/run?limit=1&offset=1&as_of={d}"),
    )
    .await;
    assert_eq!(
        (strings(col(&r, 0)), r["total"].as_i64()),
        (vec!["Q2".into()], Some(3))
    );
    let export = |q: String| {
        let app = app.clone();
        let uri = format!("/api/v1/custom-reports/{id}/export?format=csv{q}");
        async move {
            let (status, _, b) = raw(&app, Who::Staff(SALES), Method::GET, &uri, None).await;
            assert_eq!(status, StatusCode::OK);
            String::from_utf8(b).unwrap()
        }
    };
    let on_d = export(format!("&as_of={d}")).await;
    assert!(on_d.contains("Q2") && on_d.contains("Q4"), "{on_d}");
    assert!(!export(String::new()).await.contains("Q2"));
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn as_of_holds_relative_months_across_a_month_end(db: PgPool) {
    let day = |m, d| chrono::NaiveDate::from_ymd_opt(2026, m, d).unwrap();
    let (app, clock) = app_with_clock(db.clone(), day(11, 30)).await;
    seed(&db).await;
    // Q3 in October, Q1 on the last day of November, Q2 on the first of December.
    set_quote_day(&db, "Q3", day(10, 31)).await;
    set_quote_day(&db, "Q1", day(11, 30)).await;
    set_quote_day(&db, "Q2", day(12, 1)).await;
    set_quote_day(&db, "Q4", day(1, 1)).await;
    let month = |rel: &str, as_of: Option<&str>| {
        let mut body = json!({"definition": {
            "object": "quotation",
            "columns": [{"field": "quotation.number"}],
            "filters": [{"field": "quotation.quote_date", "op": "eq", "value": {"relative": rel}}],
            "sort": [{"column": "quotation.number"}]
        }});
        if let Some(a) = as_of {
            body["as_of"] = json!(a);
        }
        let app = app.clone();
        async move {
            let (status, r) = post(&app, SALES, "/custom-reports/run", body).await;
            assert_eq!(status, StatusCode::OK, "{r}");
            (
                r["as_of"].as_str().unwrap().to_string(),
                strings(col(&r, 0)),
            )
        }
    };

    // On Nov 30: this month is November, last month October.
    assert_eq!(
        month("this_month", None).await,
        ("2026-11-30".into(), vec!["Q1".to_string()])
    );
    assert_eq!(month("last_month", None).await.1, ["Q3"]);

    // On Dec 1 the anchor of the first page keeps November / October ...
    clock.set(day(12, 1));
    assert_eq!(month("this_month", Some("2026-11-30")).await.1, ["Q1"]);
    assert_eq!(month("last_month", Some("2026-11-30")).await.1, ["Q3"]);
    // ... while a fresh run moves on: this month is December, last month November.
    assert_eq!(
        month("this_month", None).await,
        ("2026-12-01".into(), vec!["Q2".to_string()])
    );
    assert_eq!(month("last_month", None).await.1, ["Q1"]);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn invalid_as_of_is_422(db: PgPool) {
    let app = setup(db.clone()).await;
    seed(&db).await;
    let def = json!({"object": "quotation", "columns": [{"field": "quotation.number"}]});
    let today = chrono::Utc::now().date_naive();
    let far = (today - chrono::Duration::days(400)).to_string();
    let future = (today + chrono::Duration::days(5)).to_string();
    for bad in ["yesterday", "2026-13-40", "", far.as_str(), future.as_str()] {
        let (status, r) = post(
            &app,
            SALES,
            "/custom-reports/run",
            json!({"definition": def, "as_of": bad}),
        )
        .await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{bad}: {r}");
    }
    let (_, created) = post(
        &app,
        SALES,
        "/custom-reports",
        json!({"name": "R", "definition": def}),
    )
    .await;
    let id = created["id"].as_str().unwrap();
    let (status, _) = get(&app, SALES, &format!("/custom-reports/{id}/run?as_of=nope")).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn invalid_relative_dates_are_422(db: PgPool) {
    let app = setup(db.clone()).await;
    seed(&db).await;
    let bad = |field: &str, op: &str, value: Value| {
        run(
            &app,
            SALES,
            json!({"object": "quotation", "columns": [{"field": "quotation.number"}],
                   "filters": [{"field": field, "op": op, "value": value}]}),
        )
    };
    for (field, op, value) in [
        (
            "quotation.quote_date",
            "eq",
            json!({"relative": "tomorrow"}),
        ),
        (
            "quotation.quote_date",
            "eq",
            json!({"relative": "'; DROP TABLE sales.quotations; --"}),
        ),
        (
            "quotation.quote_date",
            "eq",
            json!({"relative": "last_n_days", "n": 0}),
        ),
        (
            "quotation.quote_date",
            "eq",
            json!({"relative": "last_n_days", "n": 3661}),
        ),
        (
            "quotation.quote_date",
            "eq",
            json!({"relative": "last_n_days"}),
        ),
        ("quotation.quote_date", "in", json!([{"relative": "today"}])),
        (
            "quotation.quote_date",
            "is_null",
            json!({"relative": "today"}),
        ),
        ("quotation.total", "gte", json!({"relative": "today"})),
        ("quotation.number", "eq", json!({"relative": "today"})),
    ] {
        let (status, body) = bad(field, op, value.clone()).await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{value}: {body}");
    }
    // Saving validates too.
    let (status, _) = post(
        &app,
        SALES,
        "/custom-reports",
        json!({"name": "Bad", "definition": {"object": "quotation",
            "columns": [{"field": "quotation.number"}],
            "filters": [{"field": "quotation.quote_date", "op": "eq", "value": {"relative": "soon"}}]}}),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    let (_, ok) = bad(
        "quotation.quote_date",
        "eq",
        json!({"relative": "this_year"}),
    )
    .await;
    assert!(ok["rows"].is_array());
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn sibling_joins_do_not_multiply_counts_or_sums(db: PgPool) {
    let app = setup(db.clone()).await;
    seed(&db).await;
    // Alpha now has three quotations (450) and three orders (300 + 10 + 20): nine joined rows.
    for (number, quotation, total) in [("O5", "Q1", "10.00"), ("O6", "Q3", "20.00")] {
        sqlx::query(
            "INSERT INTO sales.orders (id, company_id, number, quotation_id, customer_id, customer_name,
                order_date, currency, subtotal, vat_total, total, salesperson_id, created_by)
             SELECT gen_random_uuid(), $1::uuid, $2, q.id, $3::uuid, 'Alpha Installaties NV', '2027-03-01',
                'EUR', $4::numeric, 0, $4::numeric, $5::uuid, $5::uuid
             FROM sales.quotations q WHERE q.company_id = $1::uuid AND q.number = $6",
        )
        .bind(COMPANY)
        .bind(number)
        .bind(ALPHA)
        .bind(total)
        .bind(ADMIN_ID)
        .bind(quotation)
        .execute(&db)
        .await
        .unwrap();
    }
    let def = |group: bool| {
        let mut d = json!({"object": "customer", "joins": ["quotation", "order"],
            "columns": [{"field": "quotation.total", "summary": "sum"},
                        {"field": "quotation.records", "summary": "count"},
                        {"field": "order.total", "summary": "sum"},
                        {"field": "order.total", "summary": "avg"},
                        {"field": "order.records", "summary": "count"},
                        {"field": "customer.records", "summary": "count"}]});
        if group {
            d["group_by"] = json!([{"field": "customer.name"}]);
            d["filters"] =
                json!([{"field": "customer.name", "op": "eq", "value": "Alpha Installaties NV"}]);
        } else {
            d["filters"] =
                json!([{"field": "customer.name", "op": "eq", "value": "Alpha Installaties NV"}]);
        }
        d
    };
    // Grouped: each object's records are counted and summed once.
    let (status, r) = run(&app, SALES, def(true)).await;
    assert_eq!(status, StatusCode::OK, "{r}");
    let row = &r["rows"][0];
    assert_eq!(row[1].as_f64(), Some(450.0));
    assert_eq!(row[2].as_f64(), Some(3.0));
    assert_eq!(row[3].as_f64(), Some(330.0));
    assert_eq!(row[4].as_f64(), Some(110.0));
    assert_eq!(row[5].as_f64(), Some(3.0));
    assert_eq!(row[6].as_f64(), Some(1.0));
    // The totals row agrees.
    let t = &r["totals"];
    assert_eq!(t[1].as_f64(), Some(450.0));
    assert_eq!(t[3].as_f64(), Some(330.0));
    assert_eq!(t[5].as_f64(), Some(3.0));
    assert_eq!(t[6].as_f64(), Some(1.0));
    // Single row (no grouping) too.
    let (status, r) = run(&app, SALES, def(false)).await;
    assert_eq!(status, StatusCode::OK, "{r}");
    let row = &r["rows"][0];
    assert_eq!(row[0].as_f64(), Some(450.0));
    assert_eq!(row[1].as_f64(), Some(3.0));
    assert_eq!(row[2].as_f64(), Some(330.0));
    assert_eq!(row[4].as_f64(), Some(3.0));
    assert_eq!(row[5].as_f64(), Some(1.0));
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn joined_rows_page_in_a_stable_order(db: PgPool) {
    let app = setup(db.clone()).await;
    seed(&db).await;
    let def = json!({"object": "customer", "joins": ["quotation"],
        "columns": [{"field": "customer.name"}, {"field": "quotation.number"}],
        "filters": [{"field": "customer.name", "op": "eq", "value": "Alpha Installaties NV"}]});
    let mut seen = Vec::new();
    for page in 0..3 {
        let (status, r) = post(
            &app,
            SALES,
            "/custom-reports/run",
            json!({"definition": def, "limit": 1, "offset": page}),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{r}");
        seen.extend(strings(col(&r, 1)));
    }
    seen.sort();
    assert_eq!(seen, ["Q1", "Q2", "Q3"]);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn an_admin_can_unshare_a_colleagues_report_and_gets_it_back(db: PgPool) {
    let app = setup(db.clone()).await;
    seed(&db).await;
    let def = json!({"object": "quotation", "columns": [{"field": "quotation.number"}]});
    let (_, created) = post(
        &app,
        SALES,
        "/custom-reports",
        json!({"name": "Shared one", "definition": def, "shared": true}),
    )
    .await;
    let id = created["id"].as_str().unwrap();
    // The admin makes it private to its owner: the update succeeds and returns the report.
    let (status, body) = put(
        &app,
        ADMIN,
        &format!("/custom-reports/{id}"),
        json!({"name": "Shared one", "definition": def, "shared": false}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["shared"], false);
    assert_eq!(body["mine"], false);
    assert_eq!(body["owner_name"], "Sales User");
    // Afterwards it is private again for everybody but the owner.
    assert_eq!(
        get(&app, ADMIN, &format!("/custom-reports/{id}")).await.0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        get(&app, SALES, &format!("/custom-reports/{id}")).await.0,
        StatusCode::OK
    );
}
