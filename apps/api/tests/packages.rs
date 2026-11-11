//! Incoming packages (ADR 0030, phase 1): registering parcels at the dock, photos, opening them,
//! matching with announced repairs or creating repairs from them, routing, who may do which step,
//! retries, and isolation between companies.

mod common;

use axum::http::{Method, StatusCode};
use common::{call, jpeg, raw, setup, Who};
use serde_json::{json, Value};
use sqlx::PgPool;

const ALPHA: &str = "00000000-0000-7000-8000-000000000401";
const BETA: &str = "00000000-0000-7000-8000-000000000402";
const ELECTRONICS: &str = "00000000-0000-7000-8000-000000000701";
const ADMIN: &str = "admin@example.com";
const GOODS: &str = "goods_in@example.com";
const INTAKE: &str = "intake@example.com";
const SUPPORT: &str = "support@example.com";
const SALES: &str = "sales@example.com";
const VIEWER: &str = "viewer@example.com";
const WORKSHOP: &str = "electronics@example.com";
const TECHNICIAN: &str = "technician@example.com";
const DISPATCHER: &str = "dispatcher@example.com";
const LOGISTICS: &str = "logistics@example.com";
const OTHER: &str = "other@example.com";

async fn post(app: &axum::Router, user: &str, uri: &str, body: Value) -> (StatusCode, Value) {
    call(app, user, Method::POST, uri, Some(body)).await
}
async fn put(app: &axum::Router, user: &str, uri: &str, body: Value) -> (StatusCode, Value) {
    call(app, user, Method::PUT, uri, Some(body)).await
}
async fn get(app: &axum::Router, user: &str, uri: &str) -> (StatusCode, Value) {
    call(app, user, Method::GET, uri, None).await
}
async fn delete(app: &axum::Router, user: &str, uri: &str) -> (StatusCode, Value) {
    call(app, user, Method::DELETE, uri, None).await
}

fn pid(d: &Value) -> &str {
    d["package"]["id"].as_str().unwrap()
}
fn rid(d: &Value) -> &str {
    d["repair"]["id"].as_str().unwrap()
}

/// Registers a parcel as the dock does.
async fn register(app: &axum::Router, extra: Value) -> Value {
    let mut body = json!({ "customer_id": ALPHA, "carrier": "DHL", "tracking_number": "JVGL0001" });
    body.as_object_mut()
        .unwrap()
        .extend(extra.as_object().unwrap().clone());
    let (status, d) = post(app, GOODS, "/api/v1/packages", body).await;
    assert_eq!(status, StatusCode::CREATED, "{d}");
    d
}

/// Registers and opens a parcel as a repair parcel.
async fn opened(app: &axum::Router, extra: Value) -> Value {
    let p = register(app, extra).await;
    let (status, d) = post(
        app,
        INTAKE,
        &format!("/api/v1/packages/{}/open", pid(&p)),
        json!({ "kind": "repair" }),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{d}");
    d
}

/// A repair the customer announced.
async fn announce(app: &axum::Router, extra: Value) -> Value {
    let mut body = json!({ "customer_id": ALPHA, "product_description": "Pump 5 kW", "serial_number": "SN-77",
                           "fault_description": "Does not start", "announced": true });
    body.as_object_mut()
        .unwrap()
        .extend(extra.as_object().unwrap().clone());
    let (status, r) = post(app, SUPPORT, "/api/v1/repairs", body).await;
    assert_eq!(status, StatusCode::CREATED, "{r}");
    assert_eq!(r["repair"]["status"], "announced");
    r
}

fn events(d: &Value) -> Vec<String> {
    d["events"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| e["kind"].as_str().unwrap().to_string())
        .collect()
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn the_dock_registers_parcels_and_a_retry_returns_the_first(db: PgPool) {
    let app = setup(db.clone()).await;
    let key = "00000000-0000-7000-8000-0000000000e1";
    let body = json!({ "client_id": key, "customer_id": ALPHA, "sender_name": "Alpha NV", "carrier": "bpost",
                       "tracking_number": " 3SABC123 ", "parcel_count": 2, "condition": "damaged",
                       "condition_note": "corner crushed", "customer_reference": "RMA-9" });
    let (status, d) = post(&app, GOODS, "/api/v1/packages", body.clone()).await;
    assert_eq!(status, StatusCode::CREATED, "{d}");
    let p = &d["package"];
    assert!(p["number"].as_str().unwrap().starts_with("PK-"), "{p}");
    assert!(p["number"].as_str().unwrap().ends_with("-00001"), "{p}");
    assert_eq!(p["status"], "registered");
    assert_eq!(p["kind"], Value::Null);
    assert_eq!(p["tracking_number"], "3SABC123");
    assert_eq!(p["parcel_count"], 2);
    assert_eq!(p["condition"], "damaged");
    assert_eq!(p["customer_name"], "Alpha Installaties NV");
    assert_eq!(p["received_by_name"], "Gert Goods-In");
    assert_eq!(events(&d), ["registered"]);

    // The same key again (a retry from the app) returns the first parcel and creates nothing.
    let (status, again) = post(&app, GOODS, "/api/v1/packages", body).await;
    assert_eq!(status, StatusCode::OK, "{again}");
    assert_eq!(pid(&again), pid(&d));
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM repairs.packages")
        .fetch_one(&db)
        .await
        .unwrap();
    assert_eq!(count, 1);

    // A second parcel gets the next number; one without a key can be registered twice.
    let second = register(&app, json!({})).await;
    assert!(second["package"]["number"]
        .as_str()
        .unwrap()
        .ends_with("-00002"));

    // A sender that is only a name on the label is fine.
    let (status, _) = post(
        &app,
        GOODS,
        "/api/v1/packages",
        json!({ "sender_name": "Mr. Peeters" }),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);

    // Bad input is refused.
    for bad in [
        json!({}),
        json!({ "tracking_number": "  " }),
        json!({ "customer_id": ALPHA, "condition": "burning" }),
        json!({ "customer_id": ALPHA, "parcel_count": 0 }),
        json!({ "customer_id": ALPHA, "parcel_count": 100 }),
        json!({ "customer_id": "00000000-0000-7000-8000-0000000000aa" }),
    ] {
        let (status, e) = post(&app, GOODS, "/api/v1/packages", bad.clone()).await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{bad} {e}");
    }
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn every_role_gets_the_rights_of_its_step(db: PgPool) {
    let app = setup(db).await;
    let body = json!({ "customer_id": ALPHA, "tracking_number": "T1" });
    // Registering: the dock, intake and logistics; nobody else.
    for u in [SUPPORT, SALES, VIEWER, WORKSHOP, TECHNICIAN, DISPATCHER] {
        assert_eq!(
            post(&app, u, "/api/v1/packages", body.clone()).await.0,
            StatusCode::FORBIDDEN,
            "{u}"
        );
    }
    for u in [GOODS, INTAKE, LOGISTICS, ADMIN] {
        assert_eq!(
            post(&app, u, "/api/v1/packages", body.clone()).await.0,
            StatusCode::CREATED,
            "{u}"
        );
    }
    // Reading.
    for u in [
        GOODS, INTAKE, LOGISTICS, SUPPORT, SALES, VIEWER, DISPATCHER, ADMIN,
    ] {
        let (status, list) = get(&app, u, "/api/v1/packages").await;
        assert_eq!(status, StatusCode::OK, "{u}");
        assert_eq!(list["total"], 4, "{u}");
    }
    for u in [WORKSHOP, TECHNICIAN] {
        assert_eq!(
            get(&app, u, "/api/v1/packages").await.0,
            StatusCode::FORBIDDEN,
            "{u}"
        );
    }
    // Opening: intake, support (triage) and admin; not the dock, sales, logistics, viewer.
    let (_, list) = get(&app, ADMIN, "/api/v1/packages").await;
    let items = list["items"].as_array().unwrap();
    let uri = |i: usize| format!("/api/v1/packages/{}/open", items[i]["id"].as_str().unwrap());
    for u in [GOODS, SALES, LOGISTICS, VIEWER, DISPATCHER, WORKSHOP] {
        assert_eq!(
            post(&app, u, &uri(0), json!({ "kind": "repair" })).await.0,
            StatusCode::FORBIDDEN,
            "{u}"
        );
    }
    for (i, u) in [INTAKE, SUPPORT, ADMIN].into_iter().enumerate() {
        assert_eq!(
            post(&app, u, &uri(i), json!({ "kind": "repair" })).await.0,
            StatusCode::OK,
            "{u}"
        );
    }
    // Matching (intake right): the dock and sales cannot, even on an opened parcel.
    let link = format!(
        "/api/v1/packages/{}/repairs",
        items[0]["id"].as_str().unwrap()
    );
    let r = announce(&app, json!({})).await;
    for u in [GOODS, SALES, LOGISTICS, VIEWER, WORKSHOP] {
        assert_eq!(
            post(&app, u, &link, json!({ "repair_id": rid(&r) }))
                .await
                .0,
            StatusCode::FORBIDDEN,
            "{u}"
        );
    }
    let sugg = format!(
        "/api/v1/packages/{}/repair-suggestions",
        items[0]["id"].as_str().unwrap()
    );
    for u in [GOODS, SALES, VIEWER] {
        assert_eq!(get(&app, u, &sugg).await.0, StatusCode::FORBIDDEN, "{u}");
    }
    // The dock cannot route either.
    let (status, _) = post(
        &app,
        GOODS,
        &format!("/api/v1/repairs/{}/route", rid(&r)),
        json!({ "department_id": ELECTRONICS }),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn photos_of_the_box_are_checked_served_and_retries_are_harmless(db: PgPool) {
    let app = setup(db).await;
    let p = register(&app, json!({})).await;
    let other = register(&app, json!({ "tracking_number": "JVGL0002" })).await;
    let up = |q: &str| format!("/api/v1/packages/{}/photos{q}", pid(&p));
    let photo = jpeg(3);
    let key = "00000000-0000-7000-8000-0000000000f7";

    let (status, _, body) = raw(
        &app,
        Who::Staff(GOODS),
        Method::POST,
        &up(&format!(
            "?caption=Crushed%20corner&filename=box.jpg&client_id={key}"
        )),
        Some(("application/octet-stream", photo.clone())),
    )
    .await;
    assert_eq!(
        status,
        StatusCode::CREATED,
        "{}",
        String::from_utf8_lossy(&body)
    );
    let d: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(d["photos"][0]["id"], key);
    assert_eq!(d["photos"][0]["caption"], "Crushed corner");
    assert_eq!(d["package"]["photo_count"], 1);
    assert_eq!(events(&d), ["registered", "photo"]);

    // A retry adds no second photo and no second event.
    let (status, _, body) = raw(
        &app,
        Who::Staff(GOODS),
        Method::POST,
        &up(&format!("?client_id={key}")),
        Some(("application/octet-stream", photo.clone())),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    let d: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(d["photos"].as_array().unwrap().len(), 1);
    assert_eq!(events(&d), ["registered", "photo"]);

    // The same photo id on another parcel is refused.
    let (status, _, _) = raw(
        &app,
        Who::Staff(GOODS),
        Method::POST,
        &format!("/api/v1/packages/{}/photos?client_id={key}", pid(&other)),
        Some(("application/octet-stream", photo.clone())),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);

    // Reading: whoever may see parcels, as an image that may not be sniffed.
    let get_uri = format!("/api/v1/packages/{}/photos/{key}", pid(&p));
    let (status, headers, bytes) = raw(&app, Who::Staff(VIEWER), Method::GET, &get_uri, None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(bytes, photo);
    assert_eq!(headers["content-type"], "image/jpeg");
    assert_eq!(headers["x-content-type-options"], "nosniff");
    let (status, _, _) = raw(&app, Who::Staff(TECHNICIAN), Method::GET, &get_uri, None).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    // A photo of another parcel is not found under this one.
    let (status, _, _) = raw(
        &app,
        Who::Staff(VIEWER),
        Method::GET,
        &format!("/api/v1/packages/{}/photos/{key}", pid(&other)),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    // Things that are not photos are refused; only people who may write can upload.
    for bytes in [b"%PDF-1.7 ...".to_vec(), b"MZ\x90\x00".to_vec(), vec![]] {
        let (status, _, _) = raw(
            &app,
            Who::Staff(GOODS),
            Method::POST,
            &up("?filename=evil.jpg"),
            Some(("image/jpeg", bytes)),
        )
        .await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    }
    for u in [VIEWER, SUPPORT, SALES] {
        let (status, _, _) = raw(
            &app,
            Who::Staff(u),
            Method::POST,
            &up(""),
            Some(("image/jpeg", jpeg(9))),
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN, "{u}");
    }
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn opening_decides_what_a_parcel_is_and_refusing_closes_it_unopened(db: PgPool) {
    let app = setup(db).await;

    // Repair: opened, waiting to be linked.
    let p = register(
        &app,
        json!({ "customer_id": null, "sender_name": "Mr. Peeters" }),
    )
    .await;
    let uri = format!("/api/v1/packages/{}", pid(&p));
    let (status, d) = post(
        &app,
        INTAKE,
        &format!("{uri}/open"),
        json!({ "kind": "repair", "customer_id": ALPHA, "note": "pump inside" }),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{d}");
    assert_eq!(d["package"]["status"], "opened");
    assert_eq!(d["package"]["kind"], "repair");
    assert_eq!(d["package"]["customer_name"], "Alpha Installaties NV");
    assert_eq!(d["package"]["opened_by_name"], "Inge Intake");
    assert_eq!(events(&d), ["registered", "status"]);
    // Opened twice: no.
    let (status, _) = post(
        &app,
        INTAKE,
        &format!("{uri}/open"),
        json!({ "kind": "repair" }),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);
    // The details can still be corrected while opened.
    let (status, d) = put(
        &app,
        GOODS,
        &uri,
        json!({ "customer_id": ALPHA, "carrier": "UPS", "tracking_number": "1Z" }),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{d}");
    assert_eq!(d["package"]["carrier"], "UPS");
    assert_eq!(
        put(&app, VIEWER, &uri, json!({ "customer_id": ALPHA }))
            .await
            .0,
        StatusCode::FORBIDDEN
    );

    // Other and return: closed at once, a note is required.
    for kind in ["other", "return_to_sender"] {
        let p = register(&app, json!({})).await;
        let open = format!("/api/v1/packages/{}/open", pid(&p));
        let (status, _) = post(&app, INTAKE, &open, json!({ "kind": kind })).await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{kind}");
        let (status, d) = post(
            &app,
            INTAKE,
            &open,
            json!({ "kind": kind, "note": "spare parts order" }),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{d}");
        assert_eq!(d["package"]["status"], "closed");
        assert_eq!(d["package"]["kind"], kind);
        // A closed parcel takes no more changes and holds no repair.
        let one = announce(&app, json!({})).await;
        let (status, _) = post(
            &app,
            INTAKE,
            &format!("/api/v1/packages/{}/repairs", pid(&p)),
            json!({ "repair_id": rid(&one) }),
        )
        .await;
        assert_eq!(status, StatusCode::CONFLICT);
        assert_eq!(
            put(
                &app,
                GOODS,
                &format!("/api/v1/packages/{}", pid(&p)),
                json!({ "customer_id": ALPHA })
            )
            .await
            .0,
            StatusCode::CONFLICT
        );
    }
    let (status, _) = post(
        &app,
        INTAKE,
        &format!("{uri}/open"),
        json!({ "kind": "bogus", "note": "x" }),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);

    // Refused at the dock: sent back unopened, with a note.
    let p = register(&app, json!({})).await;
    let close = format!("/api/v1/packages/{}/close", pid(&p));
    let (status, _) = post(&app, GOODS, &close, json!({ "note": " " })).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(
        post(&app, VIEWER, &close, json!({ "note": "x" })).await.0,
        StatusCode::FORBIDDEN
    );
    let (status, d) = post(
        &app,
        GOODS,
        &close,
        json!({ "note": "not ours, wrong address" }),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{d}");
    assert_eq!(d["package"]["status"], "closed");
    assert_eq!(d["package"]["kind"], "return_to_sender");
    assert_eq!(d["package"]["opened_at"], Value::Null);
    assert_eq!(d["events"][1]["note"], "not ours, wrong address");
    assert_eq!(
        post(&app, GOODS, &close, json!({ "note": "again" }))
            .await
            .0,
        StatusCode::CONFLICT
    );
    let (status, _) = post(
        &app,
        INTAKE,
        &format!("/api/v1/packages/{}/open", pid(&p)),
        json!({ "kind": "repair" }),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);

    // An opened parcel can be closed only by someone who may open.
    let p = opened(&app, json!({})).await;
    let close = format!("/api/v1/packages/{}/close", pid(&p));
    assert_eq!(
        post(&app, GOODS, &close, json!({ "note": "empty" }))
            .await
            .0,
        StatusCode::FORBIDDEN
    );
    let (status, d) = post(&app, INTAKE, &close, json!({ "note": "empty box" })).await;
    assert_eq!(status, StatusCode::OK, "{d}");
    assert_eq!(d["package"]["kind"], "repair");
    assert_eq!(d["package"]["status"], "closed");

    // The list filters.
    let (_, closed) = get(&app, ADMIN, "/api/v1/packages?status=closed").await;
    assert_eq!(closed["total"], 4);
    let (_, open) = get(&app, ADMIN, "/api/v1/packages?status=open").await;
    assert_eq!(open["total"], 1);
    let (_, found) = get(&app, ADMIN, "/api/v1/packages?q=jvgl0001&kind=other").await;
    assert_eq!(found["total"], 1);
    assert_eq!(
        get(&app, ADMIN, "/api/v1/packages?status=bogus").await.0,
        StatusCode::UNPROCESSABLE_ENTITY
    );
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn suggestions_rank_announced_repairs_and_nothing_links_until_confirmed(db: PgPool) {
    let app = setup(db).await;
    let wanted = announce(&app, json!({ "customer_reference": "RMA-9" })).await;
    let same_customer = announce(
        &app,
        json!({ "serial_number": "SN-OTHER", "product_description": "Gearbox" }),
    )
    .await;
    let other_customer = announce(
        &app,
        json!({ "customer_id": BETA, "serial_number": "SN-BETA" }),
    )
    .await;
    // A repair that has already arrived is no suggestion.
    let arrived = post(
        &app,
        SUPPORT,
        "/api/v1/repairs",
        json!({ "customer_id": ALPHA, "fault_description": "x", "serial_number": "SN-77" }),
    )
    .await
    .1;
    assert_eq!(arrived["repair"]["status"], "received");

    // Only an opened repair parcel can be matched.
    let p = register(&app, json!({ "customer_reference": "RMA-9" })).await;
    let sugg = format!("/api/v1/packages/{}/repair-suggestions", pid(&p));
    assert_eq!(get(&app, INTAKE, &sugg).await.0, StatusCode::CONFLICT);
    post(
        &app,
        INTAKE,
        &format!("/api/v1/packages/{}/open", pid(&p)),
        json!({ "kind": "repair" }),
    )
    .await;

    let (status, s) = get(&app, INTAKE, &sugg).await;
    assert_eq!(status, StatusCode::OK, "{s}");
    let list = s["suggestions"].as_array().unwrap();
    let ids: Vec<&str> = list
        .iter()
        .map(|x| x["repair"]["id"].as_str().unwrap())
        .collect();
    assert_eq!(ids, [rid(&wanted), rid(&same_customer)], "{s}");
    assert!(list[0]["reasons"]
        .as_array()
        .unwrap()
        .contains(&json!("reference")));
    assert!(list[0]["reasons"]
        .as_array()
        .unwrap()
        .contains(&json!("customer")));
    assert!(list[0]["score"].as_i64().unwrap() > list[1]["score"].as_i64().unwrap());
    assert!(!ids.contains(&rid(&other_customer)));
    assert_eq!(
        s["proposed_customer_id"],
        Value::Null,
        "the parcel has a customer"
    );

    // A serial number read from the product moves the right one up, and finds the other customer's.
    let (_, s) = get(&app, INTAKE, &format!("{sugg}?serial=sn-beta")).await;
    let found = s["suggestions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|x| x["repair"]["id"] == rid(&other_customer))
        .expect("found by its serial number");
    assert!(found["reasons"]
        .as_array()
        .unwrap()
        .contains(&json!("serial")));
    assert_eq!(found["score"], 6);

    // Nothing was linked by looking.
    let (_, d) = get(&app, INTAKE, &format!("/api/v1/repairs/{}", rid(&wanted))).await;
    assert_eq!(d["repair"]["status"], "announced");
    let (_, d) = get(&app, INTAKE, &format!("/api/v1/packages/{}", pid(&p))).await;
    assert_eq!(d["package"]["status"], "opened");
    assert_eq!(d["package"]["repair_count"], 0);

    // A parcel without a customer: the best match proposes one.
    let nameless = register(
        &app,
        json!({ "customer_id": null, "sender_name": "Mr. Peeters" }),
    )
    .await;
    post(
        &app,
        INTAKE,
        &format!("/api/v1/packages/{}/open", pid(&nameless)),
        json!({ "kind": "repair" }),
    )
    .await;
    let (_, s) = get(
        &app,
        INTAKE,
        &format!(
            "/api/v1/packages/{}/repair-suggestions?serial=SN-BETA",
            pid(&nameless)
        ),
    )
    .await;
    assert_eq!(s["suggestions"].as_array().unwrap().len(), 1, "{s}");
    assert_eq!(s["proposed_customer_id"], BETA);
    assert_eq!(s["proposed_customer_name"], "Bêta Industrie SA");
    // And with nothing to go on there is nothing to suggest.
    let (_, s) = get(
        &app,
        INTAKE,
        &format!("/api/v1/packages/{}/repair-suggestions", pid(&nameless)),
    )
    .await;
    assert_eq!(s["suggestions"].as_array().unwrap().len(), 0);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn confirming_receives_the_announced_repair_once(db: PgPool) {
    let app = setup(db.clone()).await;
    let r = announce(&app, json!({})).await;
    let p = opened(&app, json!({})).await;
    let link = format!("/api/v1/packages/{}/repairs", pid(&p));

    let (status, d) = post(
        &app,
        INTAKE,
        &link,
        json!({ "repair_id": rid(&r), "expected_status": "announced" }),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{d}");
    assert_eq!(d["repair"]["status"], "received");
    assert!(d["repair"]["received_at"].is_string());
    let kinds = events(&d);
    assert_eq!(kinds, ["created", "status", "package_linked"]);
    assert_eq!(d["events"][1]["from_status"], "announced");
    assert_eq!(d["events"][2]["note"], p["package"]["number"]);

    let (_, pk) = get(&app, INTAKE, &format!("/api/v1/packages/{}", pid(&p))).await;
    assert_eq!(pk["package"]["status"], "linked");
    assert_eq!(pk["package"]["repair_count"], 1);
    assert_eq!(pk["repairs"][0]["number"], r["repair"]["number"]);
    assert_eq!(events(&pk), ["registered", "status", "linked", "status"]);
    assert_eq!(pk["events"][2]["repair_number"], r["repair"]["number"]);

    // The same confirmation again (a retry) changes nothing.
    let (status, d) = post(
        &app,
        INTAKE,
        &link,
        json!({ "repair_id": rid(&r), "expected_status": "announced" }),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{d}");
    assert_eq!(events(&d).len(), 3);

    // A colleague who looked at the announcement before it was received loses the race.
    let p2 = opened(&app, json!({ "tracking_number": "JVGL0002" })).await;
    let link2 = format!("/api/v1/packages/{}/repairs", pid(&p2));
    let (status, e) = post(
        &app,
        INTAKE,
        &link2,
        json!({ "repair_id": rid(&r), "expected_status": "announced" }),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT, "{e}");
    assert_eq!(e["code"], "repair_status_changed", "{e}");
    // ...whereas a second parcel with the accessories is fine without that expectation.
    let (status, d) = post(&app, INTAKE, &link2, json!({ "repair_id": rid(&r) })).await;
    assert_eq!(status, StatusCode::CREATED, "{d}");
    assert_eq!(d["repair"]["status"], "received", "no second status change");
    let (_, parcels) = get(
        &app,
        VIEWER,
        &format!("/api/v1/repairs/{}/packages", rid(&r)),
    )
    .await;
    assert_eq!(parcels.as_array().unwrap().len(), 2);

    // Unknown, foreign and finished repairs.
    let unknown = "00000000-0000-7000-8000-0000000000aa";
    assert_eq!(
        post(&app, INTAKE, &link2, json!({ "repair_id": unknown }))
            .await
            .0,
        StatusCode::NOT_FOUND
    );
    let cancelled = announce(&app, json!({})).await;
    post(
        &app,
        SUPPORT,
        &format!("/api/v1/repairs/{}/status", rid(&cancelled)),
        json!({ "status": "cancelled" }),
    )
    .await;
    assert_eq!(
        post(
            &app,
            INTAKE,
            &link2,
            json!({ "repair_id": rid(&cancelled) })
        )
        .await
        .0,
        StatusCode::CONFLICT
    );

    // Either a repair or a new one, not both and not none.
    assert_eq!(
        post(&app, INTAKE, &link2, json!({})).await.0,
        StatusCode::UNPROCESSABLE_ENTITY
    );
    assert_eq!(
        post(
            &app,
            INTAKE,
            &link2,
            json!({ "repair_id": rid(&r), "new_repair": {} })
        )
        .await
        .0,
        StatusCode::UNPROCESSABLE_ENTITY
    );
    // A parcel that is not opened takes no repair.
    let fresh = register(&app, json!({ "tracking_number": "JVGL0003" })).await;
    let another = announce(&app, json!({})).await;
    assert_eq!(
        post(
            &app,
            INTAKE,
            &format!("/api/v1/packages/{}/repairs", pid(&fresh)),
            json!({ "repair_id": rid(&another) })
        )
        .await
        .0,
        StatusCode::CONFLICT
    );

    // The audit trail has the steps.
    let audited: i64 =
        sqlx::query_scalar("SELECT count(*) FROM audit.events WHERE entity = 'package'")
            .fetch_one(&db)
            .await
            .unwrap();
    assert!(audited >= 6, "{audited}");
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn a_customer_mismatch_needs_confirmation_and_an_unknown_sender_is_filled_in(db: PgPool) {
    let app = setup(db).await;
    let r = announce(&app, json!({})).await; // ALPHA
    let p = opened(&app, json!({ "customer_id": BETA })).await;
    let link = format!("/api/v1/packages/{}/repairs", pid(&p));
    let (status, e) = post(&app, INTAKE, &link, json!({ "repair_id": rid(&r) })).await;
    assert_eq!(status, StatusCode::CONFLICT, "{e}");
    assert_eq!(e["code"], "customer_mismatch", "{e}");
    let (_, d) = get(&app, INTAKE, &format!("/api/v1/repairs/{}", rid(&r))).await;
    assert_eq!(
        d["repair"]["status"], "announced",
        "the refusal changed nothing"
    );
    let (status, d) = post(
        &app,
        INTAKE,
        &link,
        json!({ "repair_id": rid(&r), "confirm_customer_mismatch": true }),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{d}");
    assert_eq!(d["repair"]["status"], "received");

    // The same for a new repair for another customer.
    let p = opened(
        &app,
        json!({ "customer_id": BETA, "tracking_number": "X2" }),
    )
    .await;
    let link = format!("/api/v1/packages/{}/repairs", pid(&p));
    let body = json!({ "new_repair": { "customer_id": ALPHA, "product_description": "Pump" } });
    let (status, e) = post(&app, INTAKE, &link, body).await;
    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(e["code"], "customer_mismatch", "{e}");
    let count: i64 = get(&app, ADMIN, "/api/v1/repairs?status=all").await.1["total"]
        .as_i64()
        .unwrap();
    assert_eq!(count, 1, "no repair was created by the refused request");

    // A parcel without a customer takes the customer of the repair it is linked to.
    let r2 = announce(&app, json!({ "customer_id": BETA })).await;
    let p = register(
        &app,
        json!({ "customer_id": null, "sender_name": "Mr. Peeters", "tracking_number": "X3" }),
    )
    .await;
    post(
        &app,
        INTAKE,
        &format!("/api/v1/packages/{}/open", pid(&p)),
        json!({ "kind": "repair" }),
    )
    .await;
    let (status, _) = post(
        &app,
        INTAKE,
        &format!("/api/v1/packages/{}/repairs", pid(&p)),
        json!({ "repair_id": rid(&r2) }),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    let (_, pk) = get(&app, INTAKE, &format!("/api/v1/packages/{}", pid(&p))).await;
    assert_eq!(pk["package"]["customer_id"], BETA);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn a_repair_is_created_from_the_parcel_when_nothing_matches_and_then_routed(db: PgPool) {
    let app = setup(db.clone()).await;
    let p = opened(&app, json!({ "customer_reference": "RMA-5" })).await;
    let link = format!("/api/v1/packages/{}/repairs", pid(&p));
    let key = "00000000-0000-7000-8000-0000000000c1";
    let body = json!({ "client_id": key, "new_repair": { "product_description": "Controller", "serial_number": "C-1" } });

    let (status, d) = post(&app, INTAKE, &link, body.clone()).await;
    assert_eq!(status, StatusCode::CREATED, "{d}");
    assert_eq!(d["repair"]["status"], "received");
    assert_eq!(d["repair"]["source"], "package");
    assert_eq!(d["repair"]["customer_id"], ALPHA, "from the parcel");
    assert_eq!(
        d["repair"]["customer_reference"], "RMA-5",
        "from the parcel"
    );
    assert_eq!(
        d["repair"]["fault_description"],
        "To be determined at intake"
    );
    assert_eq!(d["repair"]["warranty"], "unknown");
    assert!(d["repair"]["received_at"].is_string());
    assert_eq!(events(&d), ["created", "package_linked"]);

    // A retry with the same key returns that repair; no second one is made.
    let (status, again) = post(&app, INTAKE, &link, body.clone()).await;
    assert_eq!(status, StatusCode::OK, "{again}");
    assert_eq!(rid(&again), rid(&d));
    let n: i64 = sqlx::query_scalar("SELECT count(*) FROM repairs.repairs")
        .fetch_one(&db)
        .await
        .unwrap();
    assert_eq!(n, 1);
    // The key belongs to that parcel only.
    let other = opened(&app, json!({ "tracking_number": "X9" })).await;
    let (status, _) = post(
        &app,
        INTAKE,
        &format!("/api/v1/packages/{}/repairs", pid(&other)),
        body,
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);

    // A second product in the same parcel, with its own fault.
    let (status, d2) = post(
        &app,
        INTAKE,
        &link,
        json!({ "new_repair": { "product_description": "Pump", "fault_description": "Leaks" } }),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{d2}");
    assert_eq!(d2["repair"]["fault_description"], "Leaks");
    let (_, pk) = get(&app, INTAKE, &format!("/api/v1/packages/{}", pid(&p))).await;
    assert_eq!(pk["package"]["status"], "linked");
    assert_eq!(pk["repairs"].as_array().unwrap().len(), 2);

    // Intake routes it to a department, which is the existing route action.
    let (status, routed) = post(
        &app,
        INTAKE,
        &format!("/api/v1/repairs/{}/route", rid(&d)),
        json!({ "department_id": ELECTRONICS, "note": "controller board" }),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{routed}");
    assert_eq!(routed["repair"]["status"], "routed");
    assert_eq!(routed["repair"]["department_name"], "Electronics");
    // Intake may also photograph the product and add notes on the repair.
    let (status, _, _) = raw(
        &app,
        Who::Staff(INTAKE),
        Method::POST,
        &format!("/api/v1/repairs/{}/photos", rid(&d2)),
        Some(("application/octet-stream", jpeg(5))),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);

    // A customer is needed from somewhere.
    let nameless = register(
        &app,
        json!({ "customer_id": null, "sender_name": "Mr. Peeters", "tracking_number": "N1" }),
    )
    .await;
    post(
        &app,
        INTAKE,
        &format!("/api/v1/packages/{}/open", pid(&nameless)),
        json!({ "kind": "repair" }),
    )
    .await;
    let (status, e) = post(
        &app,
        INTAKE,
        &format!("/api/v1/packages/{}/repairs", pid(&nameless)),
        json!({ "new_repair": { "product_description": "Pump" } }),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{e}");
    let (status, d) = post(
        &app,
        INTAKE,
        &format!("/api/v1/packages/{}/repairs", pid(&nameless)),
        json!({ "new_repair": { "customer_id": BETA, "product_description": "Pump" } }),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{d}");
    let (_, pk) = get(
        &app,
        INTAKE,
        &format!("/api/v1/packages/{}", pid(&nameless)),
    )
    .await;
    assert_eq!(pk["package"]["customer_name"], "Bêta Industrie SA");
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn a_parcel_can_hold_several_repairs_and_a_link_can_be_undone(db: PgPool) {
    let app = setup(db).await;
    let a = announce(&app, json!({})).await;
    let b = announce(&app, json!({ "serial_number": "SN-2" })).await;
    let p = opened(&app, json!({})).await;
    let link = format!("/api/v1/packages/{}/repairs", pid(&p));
    for r in [&a, &b] {
        assert_eq!(
            post(&app, INTAKE, &link, json!({ "repair_id": rid(r) }))
                .await
                .0,
            StatusCode::CREATED
        );
    }
    let (_, pk) = get(&app, INTAKE, &format!("/api/v1/packages/{}", pid(&p))).await;
    assert_eq!(pk["package"]["repair_count"], 2);
    assert_eq!(pk["package"]["status"], "linked");

    // The other direction: opened parcels of the customer for a repair that is still announced.
    let c = announce(&app, json!({})).await;
    let loose = opened(&app, json!({ "tracking_number": "LOOSE" })).await;
    let (status, s) = get(
        &app,
        INTAKE,
        &format!("/api/v1/repairs/{}/package-suggestions", rid(&c)),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let ids: Vec<&str> = s
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x["id"].as_str().unwrap())
        .collect();
    assert_eq!(
        ids,
        [pid(&loose)],
        "linked parcels and other customers' are not suggested"
    );
    assert_eq!(
        get(
            &app,
            GOODS,
            &format!("/api/v1/repairs/{}/package-suggestions", rid(&c))
        )
        .await
        .0,
        StatusCode::FORBIDDEN
    );

    // Unlinking one repair keeps the parcel linked; the last one reopens it. The repairs keep their status.
    let unlink = |r: &Value| format!("/api/v1/packages/{}/repairs/{}", pid(&p), rid(r));
    assert_eq!(
        delete(&app, GOODS, &unlink(&a)).await.0,
        StatusCode::FORBIDDEN
    );
    let (status, d) = delete(&app, INTAKE, &unlink(&a)).await;
    assert_eq!(status, StatusCode::OK, "{d}");
    assert_eq!(d["package"]["status"], "linked");
    assert_eq!(d["repairs"].as_array().unwrap().len(), 1);
    assert_eq!(
        delete(&app, INTAKE, &unlink(&a)).await.0,
        StatusCode::NOT_FOUND
    );
    let (_, d) = delete(&app, INTAKE, &unlink(&b)).await;
    assert_eq!(d["package"]["status"], "opened");
    assert!(
        events(&d).ends_with(&["unlinked".to_string(), "status".to_string()]),
        "{:?}",
        events(&d)
    );
    let (_, r) = get(&app, INTAKE, &format!("/api/v1/repairs/{}", rid(&a))).await;
    assert_eq!(r["repair"]["status"], "received");
    assert_eq!(
        events(&r),
        ["created", "status", "package_linked", "package_unlinked"]
    );
    // It can be linked again.
    assert_eq!(
        post(&app, INTAKE, &link, json!({ "repair_id": rid(&a) }))
            .await
            .0,
        StatusCode::CREATED
    );
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn companies_do_not_see_each_others_parcels(db: PgPool) {
    let app = setup(db.clone()).await;
    const OTHER_COMPANY: &str = "00000000-0000-7000-8000-0000000000b1";
    sqlx::query("INSERT INTO core.companies (id, name) VALUES ($1::uuid, 'Other BV')")
        .bind(OTHER_COMPANY)
        .execute(&db)
        .await
        .unwrap();
    sqlx::query("INSERT INTO core.users (id, email, display_name) VALUES (gen_random_uuid(), $1, 'Other Admin')")
        .bind(OTHER)
        .execute(&db)
        .await
        .unwrap();
    sqlx::query("INSERT INTO core.user_companies (user_id, company_id, roles, is_default) SELECT id, $1::uuid, '{admin}', true FROM core.users WHERE email = $2")
        .bind(OTHER_COMPANY)
        .bind(OTHER)
        .execute(&db)
        .await
        .unwrap();

    let mine = opened(&app, json!({})).await;
    let my_repair = announce(&app, json!({})).await;
    let key = "00000000-0000-7000-8000-0000000000d1";
    // The same client key in the other company is a different parcel, numbered from 1 again.
    let (status, theirs) = post(
        &app,
        OTHER,
        "/api/v1/packages",
        json!({ "client_id": key, "sender_name": "Them" }),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{theirs}");
    assert!(theirs["package"]["number"]
        .as_str()
        .unwrap()
        .ends_with("-00001"));
    assert_eq!(
        post(
            &app,
            GOODS,
            "/api/v1/packages",
            json!({ "client_id": key, "customer_id": ALPHA })
        )
        .await
        .0,
        StatusCode::CREATED
    );

    let (_, list) = get(&app, OTHER, "/api/v1/packages").await;
    assert_eq!(list["total"], 1);
    let uri = format!("/api/v1/packages/{}", pid(&mine));
    for (method, suffix, body) in [
        (Method::GET, "", None),
        (Method::PUT, "", Some(json!({ "sender_name": "x" }))),
        (Method::POST, "/open", Some(json!({ "kind": "repair" }))),
        (Method::POST, "/close", Some(json!({ "note": "x" }))),
        (Method::GET, "/repair-suggestions", None),
        (
            Method::POST,
            "/repairs",
            Some(json!({ "repair_id": rid(&my_repair) })),
        ),
    ] {
        let (status, _) = call(&app, OTHER, method.clone(), &format!("{uri}{suffix}"), body).await;
        assert_eq!(status, StatusCode::NOT_FOUND, "{method} {suffix}");
    }
    // Nor can a parcel of theirs take my repair.
    let their_open = post(
        &app,
        OTHER,
        &format!("/api/v1/packages/{}/open", pid(&theirs)),
        json!({ "kind": "repair" }),
    )
    .await;
    assert_eq!(their_open.0, StatusCode::OK);
    let (status, _) = post(
        &app,
        OTHER,
        &format!("/api/v1/packages/{}/repairs", pid(&theirs)),
        json!({ "repair_id": rid(&my_repair) }),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    let (_, s) = get(
        &app,
        OTHER,
        &format!("/api/v1/packages/{}/repair-suggestions", pid(&theirs)),
    )
    .await;
    assert_eq!(s["suggestions"].as_array().unwrap().len(), 0);
    // Photos neither.
    let (status, _, _) = raw(
        &app,
        Who::Staff(OTHER),
        Method::POST,
        &format!("{uri}/photos"),
        Some(("image/jpeg", jpeg(1))),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn switching_the_repairs_module_off_hides_the_parcels(db: PgPool) {
    let app = setup(db.clone()).await;
    register(&app, json!({})).await;
    assert_eq!(get(&app, GOODS, "/api/v1/packages").await.0, StatusCode::OK);
    sqlx::query("UPDATE core.companies SET disabled_modules = '{repairs}'")
        .execute(&db)
        .await
        .unwrap();
    for u in [GOODS, INTAKE, ADMIN] {
        assert_eq!(
            get(&app, u, "/api/v1/packages").await.0,
            StatusCode::FORBIDDEN,
            "{u}"
        );
    }
    assert_eq!(
        post(
            &app,
            GOODS,
            "/api/v1/packages",
            json!({ "customer_id": ALPHA })
        )
        .await
        .0,
        StatusCode::FORBIDDEN
    );
    let (_, me) = get(&app, INTAKE, "/api/v1/me").await;
    assert!(!me.to_string().contains("packages."), "{me}");
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn only_a_registered_parcel_can_be_opened_and_receiving_notes_the_ticket(db: PgPool) {
    let app = setup(db.clone()).await;
    let (status, t) = post(
        &app,
        SUPPORT,
        "/api/v1/tickets",
        json!({ "subject": "Pump broken", "language": "nl", "customer_id": ALPHA, "body": "It does not start" }),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{t}");
    let ticket = t["ticket"]["id"].as_str().unwrap();
    let r = announce(&app, json!({ "ticket_id": ticket })).await;
    let p = opened(&app, json!({})).await;

    // Opening it again must not overwrite who opened it, linked or not.
    let open = format!("/api/v1/packages/{}/open", pid(&p));
    let (status, e) = post(&app, INTAKE, &open, json!({ "kind": "repair" })).await;
    assert_eq!(status, StatusCode::CONFLICT, "{e}");
    let (status, d) = post(
        &app,
        INTAKE,
        &format!("/api/v1/packages/{}/repairs", pid(&p)),
        json!({ "repair_id": rid(&r), "expected_status": "announced" }),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{d}");
    let (status, e) = post(&app, INTAKE, &open, json!({ "kind": "other", "note": "x" })).await;
    assert_eq!(status, StatusCode::CONFLICT, "{e}");

    let notes: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM helpdesk.articles WHERE ticket_id = $1::uuid AND kind = 'note' AND body LIKE '%received in parcel%'",
    )
    .bind(ticket)
    .fetch_one(&db)
    .await
    .unwrap();
    assert_eq!(notes, 1);
}
