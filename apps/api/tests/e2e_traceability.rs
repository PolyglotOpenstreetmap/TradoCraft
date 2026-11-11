//! Traceability: from a finished unit down to its serialised components and back, with the
//! warranty data that exists, and the spare parts related to the unit's product.
//!
//! Gaps found while verifying the plan (asserted below as absent, so a future extension shows up
//! as a deliberate test change): serial units carry no lot/batch, supplier, firmware version or
//! manufacturing date. Lots exist in inventory for lot-tracked products only, and no supplier
//! is modelled.

mod common;

use axum::http::{Method, StatusCode};
use common::e2e::*;
use common::{call, setup};
use serde_json::{json, Value};
use sqlx::PgPool;

const ADMIN: &str = "admin@example.com";
const SALES: &str = "sales@example.com";
const UNIT_100: &str = "00000000-0000-7000-8000-000000000301";
const U100_0001: &str = "00000000-0000-7000-8000-000000000601";
const P_1001: &str = "00000000-0000-7000-8000-000000000602";
const P_1002: &str = "00000000-0000-7000-8000-000000000603";
const C_5001: &str = "00000000-0000-7000-8000-000000000604";

async fn serial(app: &axum::Router, id: &str) -> Value {
    let (status, d) = call(
        app,
        ADMIN,
        Method::GET,
        &format!("/api/v1/serials/{id}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{d}");
    d
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn a_unit_is_traced_to_all_its_components_and_each_component_back_to_the_unit(db: PgPool) {
    let app = setup(db).await;
    let mut flow = E2eFlow::new();
    flow.serial_unit_id = Some(U100_0001.to_string());

    let d = serial(&app, flow.serial_unit_id.as_ref().unwrap()).await;
    assert_eq!(d["unit"]["serial_number"], "U100-0001");
    let mut serials: Vec<&str> = d["components"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c["serial_number"].as_str().unwrap())
        .collect();
    serials.sort();
    assert_eq!(
        serials,
        ["C-5001", "P-1001", "P-1002"],
        "the genealogy is complete"
    );
    assert!(d["components"]
        .as_array()
        .unwrap()
        .iter()
        .all(|c| c["level"] == 1));
    assert!(d["history"]
        .as_array()
        .unwrap()
        .iter()
        .all(|h| h["removed_at"].is_null()));

    // Round trip: every child with exactly one parent leads back to the original unit.
    for child in [P_1001, P_1002, C_5001] {
        let c = serial(&app, child).await;
        assert_eq!(c["unit"]["parent_id"], U100_0001, "{c}");
        assert_eq!(c["unit"]["parent_serial_number"], "U100-0001");
        assert_eq!(c["unit"]["status"], "built_in");
        assert!(c["components"].as_array().unwrap().is_empty());
    }
    // The unit itself has no parent.
    assert!(d["unit"]["parent_id"].is_null());
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn warranty_is_traced_and_unrecorded_attributes_are_absent(db: PgPool) {
    let app = setup(db).await;

    // The seed records no warranty: unknown. After it is set the value round-trips.
    let d = serial(&app, U100_0001).await;
    assert!(d["unit"]["warranty_until"].is_null());
    assert_eq!(d["unit"]["warranty_status"], "unknown");
    let (status, _) = call(
        &app,
        ADMIN,
        Method::PUT,
        &format!("/api/v1/serials/{U100_0001}/warranty"),
        Some(json!({ "warranty_until": "2027-06-30", "warranty_note": "Standard terms" })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let d = serial(&app, U100_0001).await;
    assert_eq!(d["unit"]["warranty_until"], "2027-06-30");
    assert_eq!(d["unit"]["warranty_note"], "Standard terms");
    assert_eq!(
        d["unit"]["bom_revision"], 1,
        "built to the first BOM revision"
    );

    // Not recorded on a serial unit (future extension): lot/batch, supplier, firmware version
    // and manufacturing date are absent from the traceability record.
    for field in [
        "lot",
        "lot_number",
        "batch",
        "supplier",
        "supplier_name",
        "firmware_version",
        "manufacturing_date",
        "manufactured_at",
    ] {
        assert!(
            d["unit"].get(field).is_none(),
            "{field} is now recorded: update the test"
        );
    }
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn spare_parts_of_a_unit_come_from_the_product_relations(db: PgPool) {
    let app = setup(db).await;
    let d = serial(&app, U100_0001).await;
    let product = d["unit"]["product_id"].as_str().unwrap();
    assert_eq!(product, UNIT_100);

    let (status, rel) = call(
        &app,
        SALES,
        Method::GET,
        &format!("/api/v1/products/{product}/related"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{rel}");
    let compatible: Vec<&Value> = rel["rules"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["kind"] == "compatible")
        .collect();
    assert_eq!(compatible.len(), 1, "{rel}");
    assert_eq!(compatible[0]["b_product_code"], "PUMP-05");

    // The relation listing agrees and the pump is one of the unit's components, too.
    let (_, all) = call(
        &app,
        SALES,
        Method::GET,
        "/api/v1/product-relations?kind=compatible",
        None,
    )
    .await;
    assert!(all
        .as_array()
        .unwrap()
        .iter()
        .any(|r| r["id"] == compatible[0]["id"]));
    let pumps = d["components"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|c| c["product_code"] == "PUMP-05")
        .count();
    assert_eq!(pumps, 2);
}
