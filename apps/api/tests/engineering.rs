//! Bills of materials and serial numbers.

mod common;

use axum::http::{Method, StatusCode};
use common::{call, dec, setup};
use serde_json::{json, Value};
use sqlx::PgPool;

const UNIT_100: &str = "00000000-0000-7000-8000-000000000301";
const PUMP: &str = "00000000-0000-7000-8000-000000000303";
const HOSE: &str = "00000000-0000-7000-8000-000000000304";
const FRAME: &str = "00000000-0000-7000-8000-000000000307";
const BOLT: &str = "00000000-0000-7000-8000-000000000309";
const UNIT_BOM_R1: &str = "00000000-0000-7000-8000-000000000501";
const U100_0001: &str = "00000000-0000-7000-8000-000000000601";
const P_1001: &str = "00000000-0000-7000-8000-000000000602";
const P_1003: &str = "00000000-0000-7000-8000-000000000605";
const ALPHA: &str = "00000000-0000-7000-8000-000000000401";
const ADMIN: &str = "admin@example.com";

fn n(v: &Value) -> String {
    dec(v).normalize().to_string()
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn explodes_multi_level_bom_with_scrap(db: PgPool) {
    let app = setup(db).await;
    let (status, e) = call(
        &app,
        "viewer@example.com",
        Method::GET,
        &format!("/api/v1/products/{UNIT_100}/bom/explode?quantity=2"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{e}");
    let nodes: Vec<(String, i64, String)> = e["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| {
            (
                x["code"].as_str().unwrap().to_string(),
                x["level"].as_i64().unwrap(),
                n(&x["required_quantity"]),
            )
        })
        .collect();
    let expected = [
        ("FRAME-A", 1, "2"),
        ("PROFILE-40", 2, "14.3"), // 2 × 6.5 m × 1.10 scrap
        ("BOLT-M8", 2, "48.96"),   // 2 × 24 × 1.02 scrap
        ("PUMP-05", 1, "4"),
        ("CTRL-01", 1, "2"),
        ("BOLT-M8", 1, "16"),
    ];
    assert_eq!(
        nodes,
        expected.map(|(c, l, q)| (c.to_string(), l, q.to_string()))
    );
    assert_eq!(e["nodes"][1]["parent_index"], 0);

    let req: Vec<(String, String)> = e["requirements"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| (r["code"].as_str().unwrap().to_string(), n(&r["quantity"])))
        .collect();
    let expected = [
        ("PROFILE-40", "14.3"),
        ("BOLT-M8", "64.96"),
        ("PUMP-05", "4"),
        ("CTRL-01", "2"),
    ];
    assert_eq!(req, expected.map(|(c, q)| (c.to_string(), q.to_string())));
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn bom_changes_go_through_a_new_revision(db: PgPool) {
    let app = setup(db).await;
    // The active revision is frozen.
    let (status, _) = call(
        &app,
        ADMIN,
        Method::PUT,
        &format!("/api/v1/boms/{UNIT_BOM_R1}"),
        Some(json!({ "lines": [] })),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);

    // New draft, copied from the active revision.
    let (status, draft) = call(
        &app,
        ADMIN,
        Method::POST,
        &format!("/api/v1/products/{UNIT_100}/boms"),
        Some(json!({})),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{draft}");
    assert_eq!(draft["revision"], 2);
    assert_eq!(draft["status"], "draft");
    assert_eq!(draft["lines"].as_array().unwrap().len(), 4);
    assert_eq!(draft["lines"][0]["component_has_bom"], true);

    // Only one draft at a time.
    let (status, _) = call(
        &app,
        ADMIN,
        Method::POST,
        &format!("/api/v1/products/{UNIT_100}/boms"),
        Some(json!({})),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);

    let id = draft["id"].as_str().unwrap();
    let lines = json!({ "notes": "adds hose", "lines": [
        { "component_product_id": FRAME, "quantity": "1" },
        { "component_product_id": PUMP, "quantity": "2" },
        { "component_product_id": HOSE, "quantity": "3.5", "scrap_percent": "5" },
        { "component_product_id": BOLT, "quantity": "8" }
    ]});
    let (status, updated) = call(
        &app,
        ADMIN,
        Method::PUT,
        &format!("/api/v1/boms/{id}"),
        Some(lines),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{updated}");
    assert_eq!(updated["lines"][2]["component_code"], "HOSE-25");

    let (status, active) = call(
        &app,
        ADMIN,
        Method::POST,
        &format!("/api/v1/boms/{id}/activate"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{active}");
    assert_eq!(active["status"], "active");

    let (_, list) = call(
        &app,
        ADMIN,
        Method::GET,
        &format!("/api/v1/products/{UNIT_100}/boms"),
        None,
    )
    .await;
    let statuses: Vec<_> = list
        .as_array()
        .unwrap()
        .iter()
        .map(|b| (b["revision"].as_i64().unwrap(), b["status"].clone()))
        .collect();
    assert_eq!(statuses, vec![(2, json!("active")), (1, json!("obsolete"))]);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn invalid_structures_are_rejected(db: PgPool) {
    let app = setup(db).await;
    // FRAME-A containing UNIT-100 would close a loop UNIT-100 → FRAME-A → UNIT-100.
    let (_, draft) = call(
        &app,
        ADMIN,
        Method::POST,
        &format!("/api/v1/products/{FRAME}/boms"),
        Some(json!({})),
    )
    .await;
    let id = draft["id"].as_str().unwrap();
    let (status, _) = call(
        &app,
        ADMIN,
        Method::PUT,
        &format!("/api/v1/boms/{id}"),
        Some(json!({ "lines": [{ "component_product_id": UNIT_100, "quantity": "1" }] })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let (status, err) = call(
        &app,
        ADMIN,
        Method::POST,
        &format!("/api/v1/boms/{id}/activate"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert!(
        err["message"]
            .as_str()
            .unwrap()
            .contains("FRAME-A → UNIT-100 → FRAME-A"),
        "{err}"
    );

    // A product cannot contain itself.
    let (status, _) = call(
        &app,
        ADMIN,
        Method::PUT,
        &format!("/api/v1/boms/{id}"),
        Some(json!({ "lines": [{ "component_product_id": FRAME, "quantity": "1" }] })),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);

    // Purchased products have no BOM.
    let (status, _) = call(
        &app,
        ADMIN,
        Method::POST,
        &format!("/api/v1/products/{PUMP}/boms"),
        Some(json!({})),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);

    // Sales may read but not change BOMs.
    let (status, _) = call(
        &app,
        "sales@example.com",
        Method::POST,
        &format!("/api/v1/products/{UNIT_100}/boms"),
        Some(json!({})),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn serial_units_record_what_was_built_in(db: PgPool) {
    let app = setup(db).await;
    let serials = |p: &str| format!("/api/v1/products/{p}/serials");

    let (status, _) = call(
        &app,
        ADMIN,
        Method::POST,
        &serials(HOSE),
        Some(json!({ "serial_number": "H-1" })),
    )
    .await;
    assert_eq!(
        status,
        StatusCode::UNPROCESSABLE_ENTITY,
        "hose is not serial-tracked"
    );

    let (status, unit) = call(
        &app,
        ADMIN,
        Method::POST,
        &serials(UNIT_100),
        Some(json!({ "serial_number": "U100-0002" })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{unit}");
    assert_eq!(
        unit["unit"]["bom_revision"], 1,
        "defaults to the active BOM"
    );
    let (status, _) = call(
        &app,
        ADMIN,
        Method::POST,
        &serials(UNIT_100),
        Some(json!({ "serial_number": "U100-0002" })),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);

    let u2 = unit["unit"]["id"].as_str().unwrap().to_string();
    let attach = |unit: &str| format!("/api/v1/serials/{unit}/components");

    // Spare pump goes into the new unit.
    let (status, d) = call(
        &app,
        ADMIN,
        Method::POST,
        &attach(&u2),
        Some(json!({ "child_id": P_1003 })),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{d}");
    assert_eq!(d["components"][0]["serial_number"], "P-1003");
    assert_eq!(d["components"][0]["status"], "built_in");

    // U100-0001 already holds the BOM quantity of 2 pumps.
    let (_, p4) = call(
        &app,
        ADMIN,
        Method::POST,
        &serials(PUMP),
        Some(json!({ "serial_number": "P-1004" })),
    )
    .await;
    let p4 = p4["unit"]["id"].as_str().unwrap().to_string();
    let (status, _) = call(
        &app,
        ADMIN,
        Method::POST,
        &attach(U100_0001),
        Some(json!({ "child_id": p4 })),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);

    // A unit is not a component of another unit of the same product.
    let (status, _) = call(
        &app,
        ADMIN,
        Method::POST,
        &attach(&u2),
        Some(json!({ "child_id": U100_0001 })),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);

    // Repair: replace pump P-1001 by P-1004. History keeps the removed pump.
    let (status, d) = call(
        &app,
        ADMIN,
        Method::POST,
        &format!("/api/v1/serials/{U100_0001}/components/{P_1001}/remove"),
        Some(json!({ "child_status": "scrapped" })),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{d}");
    let (status, d) = call(
        &app,
        ADMIN,
        Method::POST,
        &attach(U100_0001),
        Some(json!({ "child_id": p4 })),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{d}");
    let current: Vec<_> = d["components"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c["serial_number"].as_str().unwrap())
        .collect();
    assert!(
        current.contains(&"P-1004") && !current.contains(&"P-1001"),
        "{current:?}"
    );
    let removed: Vec<_> = d["history"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|h| h["removed_at"].is_string())
        .collect();
    assert_eq!(removed.len(), 1);
    assert_eq!(removed[0]["serial_number"], "P-1001");
    let (_, p1001) = call(
        &app,
        ADMIN,
        Method::GET,
        &format!("/api/v1/serials/{P_1001}"),
        None,
    )
    .await;
    assert_eq!(p1001["unit"]["status"], "scrapped");

    // A built-in component cannot be delivered on its own.
    let status_url = |u: &str| format!("/api/v1/serials/{u}/status");
    let (status, _) = call(
        &app,
        ADMIN,
        Method::PUT,
        &status_url(&p4),
        Some(json!({ "status": "delivered", "customer_id": ALPHA })),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);

    // Installing needs a customer.
    let (status, _) = call(
        &app,
        ADMIN,
        Method::PUT,
        &status_url(U100_0001),
        Some(json!({ "status": "installed" })),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    let (status, d) = call(
        &app,
        ADMIN,
        Method::PUT,
        &status_url(U100_0001),
        Some(json!({ "status": "installed", "customer_id": ALPHA })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(d["unit"]["customer_name"], "Alpha Installaties NV");

    let (_, page) = call(
        &app,
        "viewer@example.com",
        Method::GET,
        &format!("/api/v1/serials?customer_id={ALPHA}"),
        None,
    )
    .await;
    assert_eq!(page["total"], 1);
}
