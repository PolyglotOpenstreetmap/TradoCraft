//! Field service: planning jobs, what a technician sees, and doing the work (tasks, time, photos).

mod common;

use axum::http::{Method, StatusCode};
use common::{call, jpeg, raw, setup, Who};
use serde_json::{json, Value};
use sqlx::PgPool;

const ALPHA: &str = "00000000-0000-7000-8000-000000000401";
const BETA: &str = "00000000-0000-7000-8000-000000000402";
const SOFIE: &str = "00000000-0000-7000-8000-000000000602"; // Alpha, mobile 0475 00 00 22
const TOM: &str = "00000000-0000-7000-8000-000000000107"; // technician@example.com
const DISPATCHER: &str = "dispatcher@example.com";
const TECH: &str = "technician@example.com";
const TECH2: &str = "tech2@example.com";
const SUPPORT: &str = "support@example.com";
const VIEWER: &str = "viewer@example.com";

async fn post(app: &axum::Router, user: &str, uri: &str, body: Value) -> (StatusCode, Value) {
    call(app, user, Method::POST, uri, Some(body)).await
}
async fn get(app: &axum::Router, user: &str, uri: &str) -> (StatusCode, Value) {
    call(app, user, Method::GET, uri, None).await
}

/// A second technician, who must not see Tom's jobs.
async fn second_technician(db: &PgPool) -> String {
    let id = uuid::Uuid::now_v7();
    sqlx::query(
        "INSERT INTO core.users (id, email, display_name) VALUES ($1, $2, 'Tina Technician')",
    )
    .bind(id)
    .bind(TECH2)
    .execute(db)
    .await
    .unwrap();
    sqlx::query("INSERT INTO core.user_companies (user_id, company_id, roles, is_default) VALUES ($1, '00000000-0000-7000-8000-000000000001', '{technician}', true)")
        .bind(id).execute(db).await.unwrap();
    id.to_string()
}

async fn plan(app: &axum::Router, extra: Value) -> Value {
    let mut body = json!({
        "title": "Service pump station", "kind": "maintenance", "customer_id": ALPHA, "contact_id": SOFIE,
        "instructions": "Replace the seals. Check pressure.", "technician_ids": [TOM],
        "scheduled_start": "2026-11-09T07:00:00Z", "scheduled_end": "2026-11-09T11:00:00Z",
        "tasks": ["Isolate the pump", "Replace seals", "Pressure test"]
    });
    body.as_object_mut()
        .unwrap()
        .extend(extra.as_object().unwrap().clone());
    let (status, j) = post(app, DISPATCHER, "/api/v1/field/jobs", body).await;
    assert_eq!(status, StatusCode::CREATED, "{j}");
    j
}

fn jid(j: &Value) -> &str {
    j["job"]["id"].as_str().unwrap()
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn a_planner_says_where_to_go_what_to_do_and_who_goes(db: PgPool) {
    let app = setup(db).await;
    let j = plan(&app, json!({})).await;
    let job = &j["job"];
    assert!(job["number"].as_str().unwrap().starts_with('J'));
    assert_eq!(job["status"], "planned");
    // No address given: the customer's, ready for navigation.
    assert_eq!(job["address"], "Havenlaan 10, 2000 Antwerpen");
    assert!(
        job["maps_url"].as_str().unwrap().contains("Havenlaan%2010"),
        "{}",
        job["maps_url"]
    );
    assert_eq!(job["customer_name"], "Alpha Installaties NV");
    assert_eq!(job["contact_name"], "Sofie Maes");
    assert_eq!(job["technicians"][0]["display_name"], "Tom Technician");
    assert_eq!(job["tasks_total"], 3);
    assert_eq!(job["tasks_done"], 0);
    // The person to call when arriving.
    assert_eq!(j["contact_phones"][0]["number"], "0475 00 00 22");
    assert_eq!(j["tasks"][1]["description"], "Replace seals");

    // An own site with coordinates wins over the customer's address.
    let j2 = plan(&app, json!({ "street": "Kaai 5", "city": "Gent", "postal_code": "9000", "latitude": 51.05, "longitude": 3.72, "tasks": [] })).await;
    assert_eq!(j2["job"]["address"], "Kaai 5, 9000 Gent");
    assert!(j2["job"]["maps_url"]
        .as_str()
        .unwrap()
        .ends_with("51.05%2C3.72"));

    // Validation, and who may plan.
    for (patch, what) in [
        (json!({ "title": " " }), "title"),
        (json!({ "kind": "party" }), "kind"),
        (
            json!({ "contact_id": "00000000-0000-7000-8000-000000000604" }),
            "contact of another customer",
        ),
        (
            json!({ "scheduled_start": "2026-11-09T10:00:00Z", "scheduled_end": "2026-11-09T08:00:00Z" }),
            "ends before start",
        ),
        (json!({ "latitude": 1.0 }), "half a position"),
        (
            json!({ "technician_ids": ["00000000-0000-7000-8000-0000000000aa"] }),
            "unknown technician",
        ),
        (
            json!({ "repair_id": "00000000-0000-7000-8000-0000000000aa" }),
            "unknown repair",
        ),
    ] {
        let mut b = json!({ "title": "x", "customer_id": ALPHA });
        b.as_object_mut()
            .unwrap()
            .extend(patch.as_object().unwrap().clone());
        assert_eq!(
            post(&app, DISPATCHER, "/api/v1/field/jobs", b).await.0,
            StatusCode::UNPROCESSABLE_ENTITY,
            "{what}"
        );
    }
    for u in [TECH, SUPPORT, VIEWER] {
        assert_eq!(
            post(
                &app,
                u,
                "/api/v1/field/jobs",
                json!({ "title": "x", "customer_id": ALPHA })
            )
            .await
            .0,
            StatusCode::FORBIDDEN,
            "{u}"
        );
    }

    // Changing it: another technician, another time.
    let uri = format!("/api/v1/field/jobs/{}", jid(&j));
    let (status, u) = call(&app, DISPATCHER, Method::PUT, &uri, Some(json!({
        "title": "Service pump station (moved)", "customer_id": ALPHA, "technician_ids": [], "scheduled_start": "2026-11-10T07:00:00Z" }))).await;
    assert_eq!(status, StatusCode::OK, "{u}");
    assert_eq!(u["job"]["technicians"], json!([]));
    assert_eq!(u["job"]["title"], "Service pump station (moved)");
    let (_, unassigned) = get(&app, DISPATCHER, "/api/v1/field/jobs?technician=none").await;
    assert_eq!(
        unassigned.as_array().unwrap().len(),
        1,
        "only the one we just unassigned"
    );
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn a_technician_sees_only_their_own_jobs_in_the_order_of_their_day(db: PgPool) {
    let app = setup(db.clone()).await;
    second_technician(&db).await;
    let tina: String = sqlx::query_scalar("SELECT id::text FROM core.users WHERE email = $1")
        .bind(TECH2)
        .fetch_one(&db)
        .await
        .unwrap();
    let late = plan(&app, json!({ "title": "Afternoon job", "scheduled_start": "2026-11-09T13:00:00Z", "scheduled_end": "2026-11-09T15:00:00Z" })).await;
    let early = plan(&app, json!({ "title": "Morning job", "scheduled_start": "2026-11-09T07:00:00Z", "scheduled_end": "2026-11-09T09:00:00Z" })).await;
    let others = plan(
        &app,
        json!({ "title": "Tina's job", "technician_ids": [tina] }),
    )
    .await;
    let _both = plan(&app, json!({ "title": "Shared job", "technician_ids": [TOM, tina], "scheduled_start": "2026-11-10T08:00:00Z", "scheduled_end": "2026-11-10T10:00:00Z" })).await;

    let (status, mine) = get(&app, TECH, "/api/v1/field/my-jobs").await;
    assert_eq!(status, StatusCode::OK, "{mine}");
    let titles: Vec<&str> = mine
        .as_array()
        .unwrap()
        .iter()
        .map(|j| j["title"].as_str().unwrap())
        .collect();
    assert_eq!(
        titles,
        ["Morning job", "Afternoon job", "Shared job"],
        "by appointment"
    );

    // The list endpoint is narrowed for technicians too; and jobs of others are simply not there.
    let (_, all) = get(&app, TECH, "/api/v1/field/jobs").await;
    assert_eq!(all.as_array().unwrap().len(), 3);
    assert_eq!(
        get(&app, TECH, &format!("/api/v1/field/jobs/{}", jid(&others)))
            .await
            .0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        get(&app, TECH, &format!("/api/v1/field/jobs/{}", jid(&early)))
            .await
            .0,
        StatusCode::OK
    );
    assert_eq!(
        get(&app, TECH2, &format!("/api/v1/field/jobs/{}", jid(&early)))
            .await
            .0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        post(
            &app,
            TECH2,
            &format!("/api/v1/field/jobs/{}/start", jid(&early)),
            json!({})
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    let (_, tinas) = get(&app, TECH2, "/api/v1/field/my-jobs").await;
    assert_eq!(tinas.as_array().unwrap().len(), 2);

    // Planners, and read-only roles, see everything; the viewer sees but cannot work.
    let (_, everything) = get(&app, DISPATCHER, "/api/v1/field/jobs").await;
    assert_eq!(everything.as_array().unwrap().len(), 4);
    let (_, by_tech) = get(
        &app,
        DISPATCHER,
        &format!("/api/v1/field/jobs?technician={tina}"),
    )
    .await;
    assert_eq!(by_tech.as_array().unwrap().len(), 2);
    let (_, ranged) = get(
        &app,
        DISPATCHER,
        "/api/v1/field/jobs?from=2026-11-10T00:00:00Z&to=2026-11-11T00:00:00Z",
    )
    .await;
    assert_eq!(ranged[0]["title"], "Shared job");
    assert_eq!(
        get(&app, VIEWER, "/api/v1/field/jobs")
            .await
            .1
            .as_array()
            .unwrap()
            .len(),
        4
    );
    assert_eq!(
        post(
            &app,
            VIEWER,
            &format!("/api/v1/field/jobs/{}/start", jid(&late)),
            json!({})
        )
        .await
        .0,
        StatusCode::FORBIDDEN
    );
    // Staff without any field role have no access.
    assert_eq!(
        get(&app, SUPPORT, "/api/v1/field/my-jobs").await.0,
        StatusCode::FORBIDDEN
    );
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn a_technician_starts_ticks_tasks_logs_time_adds_photos_and_completes(db: PgPool) {
    let app = setup(db).await;
    let j = plan(&app, json!({})).await;
    let id = jid(&j);
    let base = format!("/api/v1/field/jobs/{id}");

    // Start; starting again is harmless.
    let (status, s) = post(&app, TECH, &format!("{base}/start"), json!({})).await;
    assert_eq!(status, StatusCode::OK, "{s}");
    assert_eq!(s["job"]["status"], "in_progress");
    let started = s["job"]["started_at"].clone();
    assert!(started.is_string());
    let (_, again) = post(&app, TECH, &format!("{base}/start"), json!({})).await;
    assert_eq!(again["job"]["started_at"], started);

    // Tasks: ticking is idempotent and remembers who and when; unticking is possible.
    let task = j["tasks"][0]["id"].as_str().unwrap();
    for _ in 0..2 {
        let (status, t) = post(
            &app,
            TECH,
            &format!("{base}/tasks/{task}/done"),
            json!({ "done": true }),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{t}");
        assert_eq!(t["tasks"][0]["done"], true);
        assert_eq!(t["tasks"][0]["done_by_name"], "Tom Technician");
        assert_eq!(t["job"]["tasks_done"], 1);
    }
    assert_eq!(
        post(
            &app,
            TECH,
            &format!("{base}/tasks/{task}/done"),
            json!({ "done": false })
        )
        .await
        .1["job"]["tasks_done"],
        0
    );
    assert_eq!(
        post(
            &app,
            TECH,
            &format!("{base}/tasks/00000000-0000-7000-8000-0000000000aa/done"),
            json!({ "done": true })
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    // The technician found something extra to do.
    let (status, t) = post(
        &app,
        TECH,
        &format!("{base}/tasks"),
        json!({ "description": "Replace the pressure gauge" }),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(t["job"]["tasks_total"], 4);
    assert_eq!(t["tasks"][3]["position"], 4);

    // Work and travel time. A retry with the same id adds nothing.
    let log = |kind: &str, start: &str, end: &str, id: Option<&str>| json!({ "id": id, "kind": kind, "started_at": start, "ended_at": end, "notes": "x" });
    let l1 = "00000000-0000-7000-8000-0000000000e1";
    for _ in 0..2 {
        let (status, w) = post(
            &app,
            TECH,
            &format!("{base}/work-logs"),
            log(
                "travel",
                "2026-11-05T06:00:00Z",
                "2026-11-05T06:45:00Z",
                Some(l1),
            ),
        )
        .await;
        assert_eq!(status, StatusCode::CREATED, "{w}");
        assert_eq!(w["work_logs"].as_array().unwrap().len(), 1);
    }
    let (_, w) = post(
        &app,
        TECH,
        &format!("{base}/work-logs"),
        log("work", "2026-11-05T07:00:00Z", "2026-11-05T09:30:00Z", None),
    )
    .await;
    assert_eq!(w["work_minutes"], 150);
    assert_eq!(w["travel_minutes"], 45);
    assert_eq!(w["work_logs"][0]["user_name"], "Tom Technician");
    for (b, what) in [
        (
            log("work", "2026-11-05T09:00:00Z", "2026-11-05T08:00:00Z", None),
            "ends before start",
        ),
        (
            log("work", "2026-11-01T00:00:00Z", "2026-11-03T00:00:00Z", None),
            "longer than 24 hours",
        ),
        (
            log("work", "2099-01-01T00:00:00Z", "2099-01-01T01:00:00Z", None),
            "in the future",
        ),
        (
            log("nap", "2026-11-05T07:00:00Z", "2026-11-05T08:00:00Z", None),
            "unknown kind",
        ),
    ] {
        assert_eq!(
            post(&app, TECH, &format!("{base}/work-logs"), b).await.0,
            StatusCode::UNPROCESSABLE_ENTITY,
            "{what}"
        );
    }

    // Photos from the van; the retry with the same id is harmless; junk is refused.
    let pid = "00000000-0000-7000-8000-0000000000e2";
    for _ in 0..2 {
        let (status, _, body) = raw(
            &app,
            Who::Staff(TECH),
            Method::POST,
            &format!("{base}/photos?client_id={pid}&caption=Seals%20before&filename=a.jpg"),
            Some(("image/jpeg", jpeg(1))),
        )
        .await;
        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(
            serde_json::from_slice::<Value>(&body).unwrap()["photos"]
                .as_array()
                .unwrap()
                .len(),
            1
        );
    }
    let (status, _, _) = raw(
        &app,
        Who::Staff(TECH),
        Method::POST,
        &format!("{base}/photos"),
        Some(("image/jpeg", b"not an image".to_vec())),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    let (status, h, bytes) = raw(
        &app,
        Who::Staff(DISPATCHER),
        Method::GET,
        &format!("{base}/photos/{pid}"),
        None,
    )
    .await;
    assert_eq!((status, bytes), (StatusCode::OK, jpeg(1)));
    assert_eq!(h["content-type"], "image/jpeg");

    // Finishing needs a report. Afterwards the job is closed for ticks and time, not for photos.
    assert_eq!(
        post(
            &app,
            TECH,
            &format!("{base}/complete"),
            json!({ "notes": "  " })
        )
        .await
        .0,
        StatusCode::UNPROCESSABLE_ENTITY
    );
    let (status, c) = post(
        &app,
        TECH,
        &format!("{base}/complete"),
        json!({ "notes": "Seals replaced; gauge ordered." }),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{c}");
    assert_eq!(c["job"]["status"], "completed");
    assert_eq!(
        c["job"]["completion_notes"],
        "Seals replaced; gauge ordered."
    );
    assert!(c["job"]["completed_at"].is_string());
    assert_eq!(
        post(
            &app,
            TECH,
            &format!("{base}/tasks/{task}/done"),
            json!({ "done": true })
        )
        .await
        .0,
        StatusCode::CONFLICT
    );
    assert_eq!(
        post(
            &app,
            TECH,
            &format!("{base}/work-logs"),
            log("work", "2026-11-05T10:00:00Z", "2026-11-05T11:00:00Z", None)
        )
        .await
        .0,
        StatusCode::CONFLICT
    );
    assert_eq!(
        post(
            &app,
            TECH,
            &format!("{base}/complete"),
            json!({ "notes": "again" })
        )
        .await
        .0,
        StatusCode::CONFLICT
    );
    let (status, _, _) = raw(
        &app,
        Who::Staff(TECH),
        Method::POST,
        &format!("{base}/photos?caption=Late%20upload"),
        Some(("image/jpeg", jpeg(2))),
    )
    .await;
    assert_eq!(
        status,
        StatusCode::CREATED,
        "photos taken on site may be uploaded afterwards"
    );

    // The technician cannot cancel or reopen; the planner can.
    assert_eq!(
        post(&app, TECH, &format!("{base}/reopen"), json!({}))
            .await
            .0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        post(&app, TECH, &format!("{base}/cancel"), json!({}))
            .await
            .0,
        StatusCode::FORBIDDEN
    );
    let (status, r) = post(&app, DISPATCHER, &format!("{base}/reopen"), json!({})).await;
    assert_eq!(status, StatusCode::OK, "{r}");
    assert_eq!(r["job"]["status"], "in_progress", "it was started");
    assert_eq!(
        post(&app, DISPATCHER, &format!("{base}/reopen"), json!({}))
            .await
            .0,
        StatusCode::CONFLICT,
        "not finished"
    );
    let (_, x) = post(
        &app,
        DISPATCHER,
        &format!("{base}/cancel"),
        json!({ "reason": "Customer cancelled" }),
    )
    .await;
    assert_eq!(x["job"]["status"], "cancelled");
    let (status, _, _) = raw(
        &app,
        Who::Staff(TECH),
        Method::POST,
        &format!("{base}/photos"),
        Some(("image/jpeg", jpeg(3))),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT, "no photos on a cancelled job");
    assert_eq!(
        post(&app, TECH, &format!("{base}/start"), json!({}))
            .await
            .0,
        StatusCode::CONFLICT
    );

    // Beta's technician-visible data stays Beta's: a job for another customer is just another job.
    let other = plan(
        &app,
        json!({ "customer_id": BETA, "contact_id": null, "technician_ids": [] }),
    )
    .await;
    assert_eq!(other["job"]["address"], "Rue de l'Usine 5, 4000 Liège");
    assert_eq!(
        get(&app, TECH, &format!("/api/v1/field/jobs/{}", jid(&other)))
            .await
            .0,
        StatusCode::NOT_FOUND,
        "not assigned to Tom"
    );
}
