//! Repair findings, the serial unit link and the warranty (ADR 0030, phase 2): the automatic lookup
//! against the arrival date, the workshop's void override, the final status, events and rights.

mod common;

use axum::http::{Method, StatusCode};
use common::{call, call_portal, portal_login, setup};
use serde_json::{json, Value};
use sqlx::PgPool;

const ALPHA: &str = "00000000-0000-7000-8000-000000000401";
const COMPANY: &str = "00000000-0000-7000-8000-000000000001";
const PUMP: &str = "00000000-0000-7000-8000-000000000303";
const HOSE: &str = "00000000-0000-7000-8000-000000000304";
const ELECTRONICS: &str = "00000000-0000-7000-8000-000000000701";
const ADMIN: &str = "admin@example.com";
const SUPPORT: &str = "support@example.com";
const SALES: &str = "sales@example.com";
const MANAGER: &str = "manager@example.com";
const INTAKE: &str = "intake@example.com";
const WORKSHOP: &str = "electronics@example.com";
const MECHANIC: &str = "mechanic@example.com";
const VIEWER: &str = "viewer@example.com";
const JAN: &str = "jan.peeters@alpha.example";

async fn post(app: &axum::Router, user: &str, uri: &str, body: Value) -> (StatusCode, Value) {
    call(app, user, Method::POST, uri, Some(body)).await
}
async fn put(app: &axum::Router, user: &str, uri: &str, body: Value) -> (StatusCode, Value) {
    call(app, user, Method::PUT, uri, Some(body)).await
}
async fn get(app: &axum::Router, user: &str, uri: &str) -> (StatusCode, Value) {
    call(app, user, Method::GET, uri, None).await
}

async fn unit(db: &PgPool, product: &str, serial: &str, until: Option<&str>) -> String {
    let id = uuid::Uuid::now_v7();
    sqlx::query(
        "INSERT INTO engineering.serial_units (id, company_id, product_id, serial_number, status, created_by, warranty_until)
         VALUES ($1, $2::uuid, $3::uuid, $4, 'delivered', '00000000-0000-7000-8000-000000000101',
                 $5::date)",
    )
    .bind(id)
    .bind(COMPANY)
    .bind(product)
    .bind(serial)
    .bind(until)
    .execute(db)
    .await
    .unwrap();
    id.to_string()
}

/// A repair that is already on the bench (received today).
async fn repair(app: &axum::Router, extra: Value) -> Value {
    let mut body = json!({ "customer_id": ALPHA, "product_description": "Pump", "serial_number": "SN-1",
                           "fault_description": "Does not start" });
    body.as_object_mut()
        .unwrap()
        .extend(extra.as_object().unwrap().clone());
    let (status, r) = post(app, SUPPORT, "/api/v1/repairs", body).await;
    assert_eq!(status, StatusCode::CREATED, "{r}");
    r
}

fn id(r: &Value) -> &str {
    r["repair"]["id"].as_str().unwrap()
}

fn kinds(d: &Value) -> Vec<String> {
    d["events"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| e["kind"].as_str().unwrap().to_string())
        .collect()
}

async fn warranty(app: &axum::Router, rid: &str) -> Value {
    let (status, w) = get(app, ADMIN, &format!("/api/v1/repairs/{rid}/warranty")).await;
    assert_eq!(status, StatusCode::OK, "{w}");
    w
}

fn days(db_offset: i64) -> String {
    (chrono::Utc::now().date_naive() + chrono::Duration::days(db_offset)).to_string()
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn the_warranty_is_looked_up_from_the_serial_unit_and_the_arrival_date(db: PgPool) {
    let app = setup(db.clone()).await;
    unit(&db, PUMP, "IN-1", Some(&days(30))).await;
    unit(&db, PUMP, "OUT-1", Some(&days(-30))).await;
    unit(&db, PUMP, "NODATE-1", None).await;

    let r = repair(
        &app,
        json!({ "product_id": PUMP, "serial_number": " in-1 " }),
    )
    .await;
    assert_eq!(r["repair"]["warranty"], "in_warranty");
    assert_eq!(r["repair"]["warranty_auto"], "in_warranty");
    assert_eq!(r["repair"]["warranty_overridden"], false);
    assert!(r["repair"]["serial_unit_id"].is_string());
    assert!(kinds(&r).contains(&"warranty_auto".to_string()), "{r}");
    let w = warranty(&app, id(&r)).await;
    assert_eq!(w["auto_until"], days(30));
    assert_eq!(w["compared_with"], days(0));
    assert_eq!(w["arrived"], true);
    assert_eq!(w["serial_unit"]["serial_number"], "IN-1");
    assert_eq!(w["serial_unit"]["product_code"], "PUMP-05");

    let out = repair(
        &app,
        json!({ "product_id": PUMP, "serial_number": "OUT-1" }),
    )
    .await;
    assert_eq!(out["repair"]["warranty"], "out_of_warranty");
    // No date, no unit, or no serial number at all: unknown, and nothing is written as an event.
    for extra in [
        json!({ "product_id": PUMP, "serial_number": "NODATE-1" }),
        json!({ "product_id": PUMP, "serial_number": "NOBODY" }),
        json!({ "serial_number": "" }),
    ] {
        let r = repair(&app, extra).await;
        assert_eq!(r["repair"]["warranty"], "unknown", "{r}");
        assert!(!kinds(&r).contains(&"warranty_auto".to_string()), "{r}");
    }
    // Without the product the serial number alone is accepted when it is unique.
    let r = repair(&app, json!({ "serial_number": "OUT-1" })).await;
    assert_eq!(r["repair"]["warranty"], "out_of_warranty");
    // A unit of another product does not match.
    let r = repair(
        &app,
        json!({ "product_id": HOSE, "serial_number": "OUT-1" }),
    )
    .await;
    assert_eq!(r["repair"]["warranty"], "unknown");
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn the_arrival_date_decides_not_today_and_the_last_day_counts(db: PgPool) {
    let app = setup(db.clone()).await;
    unit(&db, PUMP, "EDGE-1", Some("2026-03-01")).await;
    // Announced: provisional, compared with today (the unit's warranty has ended by then).
    let (status, r) = post(
        &app,
        SUPPORT,
        "/api/v1/repairs",
        json!({ "customer_id": ALPHA, "product_id": PUMP, "serial_number": "EDGE-1",
                "fault_description": "x", "announced": true }),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{r}");
    let rid = id(&r).to_string();
    let w = warranty(&app, &rid).await;
    assert_eq!(w["arrived"], false);
    assert_eq!(w["auto"], "out_of_warranty");

    // It arrived on the last day of the warranty (the date is backdated here).
    sqlx::query("UPDATE repairs.repairs SET received_at = '2026-03-01T23:30:00Z', status = 'received' WHERE id = $1::uuid")
        .bind(&rid)
        .execute(&db)
        .await
        .unwrap();
    let (status, w) = post(
        &app,
        SUPPORT,
        &format!("/api/v1/repairs/{rid}/warranty/recheck"),
        json!({}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{w}");
    assert_eq!(w["auto"], "in_warranty");
    assert_eq!(w["compared_with"], "2026-03-01");
    assert_eq!(w["warranty"], "in_warranty");
    // One day later it is out.
    sqlx::query(
        "UPDATE repairs.repairs SET received_at = '2026-03-02T00:10:00Z' WHERE id = $1::uuid",
    )
    .bind(&rid)
    .execute(&db)
    .await
    .unwrap();
    let (_, w) = post(
        &app,
        SUPPORT,
        &format!("/api/v1/repairs/{rid}/warranty/recheck"),
        json!({}),
    )
    .await;
    assert_eq!(w["auto"], "out_of_warranty");
    // A recheck that changes nothing writes no event.
    let before = kinds(&get(&app, ADMIN, &format!("/api/v1/repairs/{rid}")).await.1).len();
    post(
        &app,
        SUPPORT,
        &format!("/api/v1/repairs/{rid}/warranty/recheck"),
        json!({}),
    )
    .await;
    let after = kinds(&get(&app, ADMIN, &format!("/api/v1/repairs/{rid}")).await.1).len();
    assert_eq!(before, after);
    // Viewers cannot recheck.
    let (status, _) = post(
        &app,
        VIEWER,
        &format!("/api/v1/repairs/{rid}/warranty/recheck"),
        json!({}),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn receiving_an_announced_repair_runs_the_lookup_with_the_arrival_date(db: PgPool) {
    let app = setup(db.clone()).await;
    unit(&db, PUMP, "ARR-1", Some(&days(5))).await;
    let (_, r) = post(
        &app,
        SUPPORT,
        "/api/v1/repairs",
        json!({ "customer_id": ALPHA, "product_id": PUMP, "serial_number": "ARR-1",
                "fault_description": "x", "announced": true }),
    )
    .await;
    let rid = id(&r).to_string();
    assert_eq!(r["repair"]["warranty"], "in_warranty");
    let (status, d) = post(
        &app,
        SUPPORT,
        &format!("/api/v1/repairs/{rid}/status"),
        json!({ "status": "received" }),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{d}");
    let w = warranty(&app, &rid).await;
    assert_eq!(w["arrived"], true);
    assert_eq!(w["auto"], "in_warranty");
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn an_ambiguous_serial_number_needs_a_person_to_pick_the_unit(db: PgPool) {
    let app = setup(db.clone()).await;
    let pump = unit(&db, PUMP, "DUP-1", Some(&days(10))).await;
    unit(&db, HOSE, "DUP-1", Some(&days(-10))).await;
    let r = repair(&app, json!({ "serial_number": "DUP-1" })).await;
    let rid = id(&r);
    assert_eq!(r["repair"]["warranty"], "unknown");
    assert_eq!(r["repair"]["serial_unit_id"], Value::Null);
    let w = warranty(&app, rid).await;
    assert_eq!(w["candidates"].as_array().unwrap().len(), 2);

    // Intake picks the pump: the repair gets its product, and the warranty follows.
    let uri = format!("/api/v1/repairs/{rid}/serial-unit");
    for who in [SALES, VIEWER, WORKSHOP] {
        let (status, _) = put(&app, who, &uri, json!({ "serial_unit_id": pump })).await;
        assert_eq!(status, StatusCode::FORBIDDEN, "{who}");
    }
    let (status, d) = put(&app, INTAKE, &uri, json!({ "serial_unit_id": pump })).await;
    assert_eq!(status, StatusCode::OK, "{d}");
    assert_eq!(d["repair"]["serial_unit_id"], pump);
    assert_eq!(d["repair"]["product_code"], "PUMP-05");
    assert_eq!(d["repair"]["warranty"], "in_warranty");
    assert!(kinds(&d).contains(&"serial_unit".to_string()));
    assert_eq!(warranty(&app, rid).await["candidates"], json!([]));

    // Both or neither of the two fields, an unknown unit, or a unit of another product.
    assert_eq!(
        put(&app, INTAKE, &uri, json!({})).await.0,
        StatusCode::UNPROCESSABLE_ENTITY
    );
    assert_eq!(
        put(
            &app,
            INTAKE,
            &uri,
            json!({ "serial_unit_id": pump, "clear": true })
        )
        .await
        .0,
        StatusCode::UNPROCESSABLE_ENTITY
    );
    assert_eq!(
        put(
            &app,
            INTAKE,
            &uri,
            json!({ "serial_unit_id": "00000000-0000-7000-8000-00000000ffff" })
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    let hose_unit: String = sqlx::query_scalar(
        "SELECT id::text FROM engineering.serial_units WHERE product_id = $1::uuid AND serial_number = 'DUP-1'",
    )
    .bind(HOSE)
    .fetch_one(&db)
    .await
    .unwrap();
    assert_eq!(
        put(&app, INTAKE, &uri, json!({ "serial_unit_id": hose_unit }))
            .await
            .0,
        StatusCode::UNPROCESSABLE_ENTITY
    );

    // Unlinking sends the automatic result back to unknown; the final status follows.
    let (status, d) = put(&app, INTAKE, &uri, json!({ "clear": true })).await;
    assert_eq!(status, StatusCode::OK, "{d}");
    assert_eq!(d["repair"]["serial_unit_id"], Value::Null);
    assert_eq!(d["repair"]["warranty_auto"], "unknown");
    assert_eq!(d["repair"]["warranty"], "unknown");
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn changing_the_serial_number_looks_the_warranty_up_again(db: PgPool) {
    let app = setup(db.clone()).await;
    unit(&db, PUMP, "A-1", Some(&days(10))).await;
    unit(&db, PUMP, "B-1", Some(&days(-10))).await;
    let r = repair(&app, json!({ "product_id": PUMP, "serial_number": "A-1" })).await;
    let rid = id(&r);
    assert_eq!(r["repair"]["warranty"], "in_warranty");
    let form = |serial: &str, warranty: &str| {
        json!({ "product_id": PUMP, "product_description": "Pump", "serial_number": serial,
                "fault_description": "x", "warranty": warranty, "priority": "normal" })
    };
    let uri = format!("/api/v1/repairs/{rid}");
    // Saving the form unchanged is fine.
    let (status, d) = put(&app, SUPPORT, &uri, form("A-1", "in_warranty")).await;
    assert_eq!(status, StatusCode::OK, "{d}");
    // A hand-typed warranty is refused while the lookup knows it.
    let (status, _) = put(&app, SUPPORT, &uri, form("A-1", "out_of_warranty")).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    // Another serial number: the old unit is dropped and the new one decides.
    let (status, d) = put(&app, SUPPORT, &uri, form("B-1", "in_warranty")).await;
    assert_eq!(status, StatusCode::OK, "{d}");
    assert_eq!(d["repair"]["warranty"], "out_of_warranty");
    assert_eq!(d["repair"]["warranty_auto"], "out_of_warranty");
    // An unknown serial number: the lookup knows nothing, and staff may decide by hand (an event).
    let (status, d) = put(&app, SUPPORT, &uri, form("ZZZ", "out_of_warranty")).await;
    assert_eq!(status, StatusCode::OK, "{d}");
    assert_eq!(d["repair"]["warranty_auto"], "unknown");
    let (status, d) = put(&app, SUPPORT, &uri, form("ZZZ", "in_warranty")).await;
    assert_eq!(status, StatusCode::OK, "{d}");
    assert_eq!(d["repair"]["warranty"], "in_warranty");
    assert!(kinds(&d).contains(&"warranty_manual".to_string()), "{d}");
    // Void cannot be typed in.
    let (status, _) = put(&app, SUPPORT, &uri, form("ZZZ", "void")).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn the_workshop_voids_the_warranty_and_the_automatic_result_stays(db: PgPool) {
    let app = setup(db.clone()).await;
    unit(&db, PUMP, "VOID-1", Some(&days(60))).await;
    let r = repair(
        &app,
        json!({ "product_id": PUMP, "serial_number": "VOID-1" }),
    )
    .await;
    let rid = id(&r).to_string();
    assert_eq!(r["repair"]["warranty"], "in_warranty");
    let (status, _) = post(
        &app,
        SUPPORT,
        &format!("/api/v1/repairs/{rid}/route"),
        json!({ "department_id": ELECTRONICS }),
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    let uri = format!("/api/v1/repairs/{rid}/warranty/override");
    // Rights: support, sales, intake and viewers cannot; a workshop colleague of another department
    // cannot touch this repair.
    for who in [SUPPORT, SALES, INTAKE, VIEWER] {
        let (status, _) = put(&app, who, &uri, json!({ "reason": "water damage" })).await;
        assert_eq!(status, StatusCode::FORBIDDEN, "{who}");
    }
    let (status, _) = put(&app, MECHANIC, &uri, json!({ "reason": "water damage" })).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    // A reason is mandatory.
    for reason in ["", "   "] {
        let (status, _) = put(&app, WORKSHOP, &uri, json!({ "reason": reason })).await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    }
    let (status, d) = put(
        &app,
        WORKSHOP,
        &uri,
        json!({ "reason": " Water damage on the board " }),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{d}");
    assert_eq!(d["repair"]["warranty"], "void");
    assert_eq!(d["repair"]["warranty_auto"], "in_warranty");
    assert_eq!(d["repair"]["warranty_overridden"], true);
    let ev = d["events"]
        .as_array()
        .unwrap()
        .iter()
        .find(|e| e["kind"] == "warranty_void")
        .unwrap();
    assert_eq!(ev["note"], "Water damage on the board");
    assert_eq!(ev["actor_name"], "Els Electronics");
    let w = warranty(&app, &rid).await;
    assert_eq!(w["warranty"], "void");
    assert_eq!(w["auto"], "in_warranty");
    assert_eq!(w["auto_until"], days(60));
    assert_eq!(w["overridden"], true);
    assert_eq!(w["override_by_name"], "Els Electronics");
    assert_eq!(w["override_reason"], "Water damage on the board");
    assert!(w["override_at"].is_string());

    // Twice is a conflict; a recheck and a serial-unit change keep the void.
    assert_eq!(
        put(&app, WORKSHOP, &uri, json!({ "reason": "again" }))
            .await
            .0,
        StatusCode::CONFLICT
    );
    let (_, w) = post(
        &app,
        SUPPORT,
        &format!("/api/v1/repairs/{rid}/warranty/recheck"),
        json!({}),
    )
    .await;
    assert_eq!(w["warranty"], "void");
    // The hand-typed status cannot undo it either.
    let (status, _) = put(
        &app,
        SUPPORT,
        &format!("/api/v1/repairs/{rid}"),
        json!({ "product_id": PUMP, "product_description": "Pump", "serial_number": "VOID-1",
                "fault_description": "x", "warranty": "in_warranty", "priority": "normal" }),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);

    // The database refuses a void without its reason.
    let bad =
        sqlx::query("UPDATE repairs.repairs SET warranty_override_reason = '' WHERE id = $1::uuid")
            .bind(&rid)
            .execute(&db)
            .await;
    assert!(bad.is_err());

    // Removing needs the permission and a reason, restores the automatic status, keeps the events.
    let del = |who: &'static str, body: Value| {
        let app = app.clone();
        let uri = uri.clone();
        async move { call(&app, who, Method::DELETE, &uri, Some(body)).await }
    };
    assert_eq!(
        del(SUPPORT, json!({ "reason": "x" })).await.0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        del(WORKSHOP, json!({ "reason": " " })).await.0,
        StatusCode::UNPROCESSABLE_ENTITY
    );
    let (status, d) = del(
        WORKSHOP,
        json!({ "reason": "Customer proved a factory fault" }),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{d}");
    assert_eq!(d["repair"]["warranty"], "in_warranty");
    assert_eq!(d["repair"]["warranty_overridden"], false);
    let k = kinds(&d);
    assert!(k.contains(&"warranty_void".to_string()));
    assert!(k.contains(&"warranty_void_removed".to_string()));
    assert_eq!(
        del(WORKSHOP, json!({ "reason": "again" })).await.0,
        StatusCode::CONFLICT
    );
    // The sales manager (escalation owner) and the admin may void as well.
    for who in [MANAGER, ADMIN] {
        let (status, d) = put(&app, who, &uri, json!({ "reason": "misuse" })).await;
        assert_eq!(status, StatusCode::OK, "{who} {d}");
        let (status, _) = call(
            &app,
            who,
            Method::DELETE,
            &uri,
            Some(json!({ "reason": "undo" })),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
    }
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn a_void_works_on_any_automatic_result_and_not_on_a_finished_repair(db: PgPool) {
    let app = setup(db.clone()).await;
    // No serial unit at all: unknown becomes void.
    let r = repair(&app, json!({ "serial_number": "NONE" })).await;
    let rid = id(&r).to_string();
    let uri = format!("/api/v1/repairs/{rid}/warranty/override");
    let (status, d) = put(&app, MANAGER, &uri, json!({ "reason": "misuse" })).await;
    assert_eq!(status, StatusCode::OK, "{d}");
    assert_eq!(d["repair"]["warranty"], "void");
    assert_eq!(d["repair"]["warranty_auto"], "unknown");
    // Removing falls back to the automatic result, which does not know.
    let (_, d) = call(
        &app,
        MANAGER,
        Method::DELETE,
        &uri,
        Some(json!({ "reason": "undo" })),
    )
    .await;
    assert_eq!(d["repair"]["warranty"], "unknown");
    // Shipped or cancelled repairs keep their warranty.
    for (to, path) in [("cancelled", "cancel")] {
        let (status, _) = post(
            &app,
            ADMIN,
            &format!("/api/v1/repairs/{rid}/status"),
            json!({ "status": to }),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{path}");
    }
    let (status, _) = put(&app, MANAGER, &uri, json!({ "reason": "late" })).await;
    assert_eq!(status, StatusCode::CONFLICT);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn findings_are_recorded_by_the_people_who_may_work_on_the_repair(db: PgPool) {
    let app = setup(db.clone()).await;
    let r = repair(&app, json!({})).await;
    let rid = id(&r).to_string();
    assert_eq!(r["repair"]["findings"], "");
    let uri = format!("/api/v1/repairs/{rid}/findings");
    let (status, d) = put(
        &app,
        INTAKE,
        &uri,
        json!({ "findings": " Burnt smell, board discoloured " }),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{d}");
    assert_eq!(d["repair"]["findings"], "Burnt smell, board discoloured");
    assert_eq!(d["repair"]["diagnosis"], "");
    let ev = d["events"].as_array().unwrap().last().unwrap().clone();
    assert_eq!(ev["kind"], "findings");
    assert_eq!(ev["note"], "Burnt smell, board discoloured");
    // The diagnosis is kept when left out, and changes when given.
    let (_, d) = put(
        &app,
        SUPPORT,
        &uri,
        json!({ "findings": "x", "diagnosis": "Replace the board" }),
    )
    .await;
    assert_eq!(d["repair"]["diagnosis"], "Replace the board");
    let (_, d) = put(&app, SUPPORT, &uri, json!({ "findings": "y" })).await;
    assert_eq!(d["repair"]["diagnosis"], "Replace the board");
    // Viewers cannot; a workshop user of another department cannot once the repair is routed.
    assert_eq!(
        put(&app, VIEWER, &uri, json!({ "findings": "z" })).await.0,
        StatusCode::FORBIDDEN
    );
    post(
        &app,
        SUPPORT,
        &format!("/api/v1/repairs/{rid}/route"),
        json!({ "department_id": ELECTRONICS }),
    )
    .await;
    assert_eq!(
        put(&app, MECHANIC, &uri, json!({ "findings": "z" }))
            .await
            .0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        put(&app, WORKSHOP, &uri, json!({ "findings": "z" }))
            .await
            .0,
        StatusCode::OK
    );
    assert_eq!(
        put(
            &app,
            ADMIN,
            &format!(
                "/api/v1/repairs/{}/findings",
                "00000000-0000-7000-8000-00000000ffff"
            ),
            json!({ "findings": "z" })
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn the_customer_portal_does_not_see_findings_or_the_warranty_override(db: PgPool) {
    let app = setup(db.clone()).await;
    let jan = portal_login(&app, &db, JAN).await;
    let (status, d) = call_portal(
        &app,
        Some(&jan),
        Method::POST,
        "/api/v1/portal/repairs",
        Some(json!({ "product_description": "Pump", "serial_number": "P-1", "fault_description": "Leaks" })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{d}");
    let rid = d["repair"]["id"].as_str().unwrap().to_string();
    post(
        &app,
        SUPPORT,
        &format!("/api/v1/repairs/{rid}/status"),
        json!({ "status": "received" }),
    )
    .await;
    put(
        &app,
        INTAKE,
        &format!("/api/v1/repairs/{rid}/findings"),
        json!({ "findings": "SECRET-FINDING" }),
    )
    .await;
    put(
        &app,
        MANAGER,
        &format!("/api/v1/repairs/{rid}/warranty/override"),
        json!({ "reason": "SECRET-REASON" }),
    )
    .await;
    let (status, d) = call_portal(
        &app,
        Some(&jan),
        Method::GET,
        &format!("/api/v1/portal/repairs/{rid}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{d}");
    let text = d.to_string();
    assert!(!text.contains("SECRET"), "{text}");
    assert!(!text.contains("warranty"), "{text}");
    assert!(!text.contains("findings"), "{text}");
}
