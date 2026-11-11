//! Helpdesk follow-up: business hours in the SLA, automation rules, ticket split and reporting.

mod common;

use axum::http::{Method, StatusCode};
use chrono::{DateTime, Utc};
use common::{call, call_raw, raw, setup, Who};
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

const ALPHA: &str = "00000000-0000-7000-8000-000000000401";
const COMPANY: &str = "00000000-0000-7000-8000-000000000001";
const ADMIN: &str = "admin@example.com";
const SALES: &str = "sales@example.com";
const SUPPORT: &str = "support@example.com";
const VIEWER: &str = "viewer@example.com";
const TECH: &str = "technician@example.com";
const SUPPORT_ID: &str = "00000000-0000-7000-8000-000000000105";
const OTHER: &str = "other@example.com";

const PDF: &[u8] = b"%PDF-1.4\n1 0 obj<<>>endobj\ntrailer<<>>\n%%EOF\n";

fn when(v: &Value) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(v.as_str().unwrap_or_else(|| panic!("not a date: {v}")))
        .unwrap()
        .with_timezone(&Utc)
}

fn at(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s).unwrap().with_timezone(&Utc)
}

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

async fn new_ticket(app: &axum::Router, user: &str, subject: &str) -> String {
    let (status, t) = post(
        app,
        user,
        "/tickets",
        json!({ "subject": subject, "customer_id": ALPHA, "contact_email": "anna@alpha.example", "body": "First note" }),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{t}");
    t["ticket"]["id"].as_str().unwrap().to_string()
}

async fn ticket(app: &axum::Router, id: &str) -> Value {
    let (status, t) = get(app, SUPPORT, &format!("/tickets/{id}")).await;
    assert_eq!(status, StatusCode::OK, "{t}");
    t
}

/// Saves the ticket with another priority (the full update call).
async fn set_priority(app: &axum::Router, id: &str, priority: &str) -> Value {
    let d = ticket(app, id).await;
    let t = &d["ticket"];
    let (code, r) = put(
        app,
        SUPPORT,
        &format!("/tickets/{id}"),
        json!({
            "subject": t["subject"], "status": t["status"], "priority": priority,
            "category": t["category"], "language": t["language"], "owner_user_id": t["owner_user_id"],
            "customer_id": t["customer_id"], "contact_id": t["contact_id"],
            "contact_name": t["contact_name"], "contact_email": t["contact_email"],
        }),
    )
    .await;
    assert_eq!(code, StatusCode::OK, "{r}");
    r
}

async fn created_at(db: &PgPool, id: &str, utc: &str) {
    sqlx::query("UPDATE helpdesk.tickets SET created_at = $2::timestamptz WHERE id = $1::uuid")
        .bind(id)
        .bind(utc)
        .execute(db)
        .await
        .unwrap();
}

fn office_hours() -> Value {
    json!([1, 2, 3, 4, 5]
        .map(|d| json!({ "weekday": d, "start_time": "09:00", "end_time": "17:00" })))
}

async fn calendar(app: &axum::Router, enabled: bool, hours: Value) -> (StatusCode, Value) {
    put(
        app,
        ADMIN,
        "/helpdesk/calendar",
        json!({ "enabled": enabled, "time_zone": "Europe/Brussels", "hours": hours }),
    )
    .await
}

async fn due(app: &axum::Router, id: &str) -> (DateTime<Utc>, DateTime<Utc>) {
    let t = ticket(app, id).await;
    let sla = &t["ticket"]["sla"];
    (
        when(&sla["first_response"]["due_at"]),
        when(&sla["resolution"]["due_at"]),
    )
}

async fn second_company(db: &PgPool) {
    let company = Uuid::now_v7();
    sqlx::query("INSERT INTO core.companies (id, name, country_code, currency) VALUES ($1, 'Other BV', 'BE', 'EUR')")
        .bind(company)
        .execute(db)
        .await
        .unwrap();
    let id = Uuid::now_v7();
    sqlx::query("INSERT INTO core.users (id, email, display_name) VALUES ($1, $2, 'Other Admin')")
        .bind(id)
        .bind(OTHER)
        .execute(db)
        .await
        .unwrap();
    sqlx::query("INSERT INTO core.user_companies (user_id, company_id, roles, is_default) VALUES ($1, $2, '{admin}', true)")
        .bind(id)
        .bind(company)
        .execute(db)
        .await
        .unwrap();
}

// ---------------------------------------------------------------------------------------------
// Business hours
// ---------------------------------------------------------------------------------------------

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn due_times_count_business_minutes_when_the_calendar_is_on(db: PgPool) {
    let app = setup(db.clone()).await;
    // Without a calendar: plain elapsed time, as before.
    let plain = new_ticket(&app, SUPPORT, "Plain").await;
    let t = ticket(&app, &plain).await;
    assert_eq!(
        due(&app, &plain).await.0,
        when(&t["ticket"]["created_at"]) + chrono::Duration::minutes(480)
    );

    let (status, cal) = calendar(&app, true, office_hours()).await;
    assert_eq!(status, StatusCode::OK, "{cal}");
    let (status, _) = post(
        &app,
        ADMIN,
        "/helpdesk/calendar/holidays",
        json!({ "day": "2026-11-11", "name": "Armistice" }),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);

    // Friday 13 November 16:30 local (15:30 UTC): 8 h first response = 30 min on Friday, 7.5 h on
    // Monday -> Monday 16:30 local. Resolution 72 h ends on Thursday 26 November 16:30 local.
    let friday = new_ticket(&app, SUPPORT, "Friday evening").await;
    created_at(&db, &friday, "2026-11-13T15:30:00Z").await;
    // Saturday noon: counting starts on Monday 09:00 -> Monday 17:00.
    let saturday = new_ticket(&app, SUPPORT, "Saturday").await;
    created_at(&db, &saturday, "2026-11-14T11:00:00Z").await;
    // Tuesday 10 November 16:00 local, high priority (4 h / 24 h) across the holiday.
    let holiday = new_ticket(&app, SUPPORT, "Over the holiday").await;
    created_at(&db, &holiday, "2026-11-10T15:00:00Z").await;
    set_priority(&app, &holiday, "high").await;
    // Friday 26 March 2027 16:00 local (CET), the clock changes on Sunday 28 March.
    let dst = new_ticket(&app, SUPPORT, "Across the change").await;
    created_at(&db, &dst, "2027-03-26T15:00:00Z").await;
    // Save the same calendar again: every open ticket is recomputed from its creation time.
    let (status, _) = calendar(&app, true, office_hours()).await;
    assert_eq!(status, StatusCode::OK);

    let (fr, res) = due(&app, &friday).await;
    assert_eq!(fr, at("2026-11-16T15:30:00Z"));
    assert_eq!(res, at("2026-11-26T15:30:00Z"));
    assert_eq!(due(&app, &saturday).await.0, at("2026-11-16T16:00:00Z"));
    let (fr, res) = due(&app, &holiday).await;
    assert_eq!(
        fr,
        at("2026-11-12T11:00:00Z"),
        "1 h Tuesday, holiday Wednesday, 3 h Thursday"
    );
    assert_eq!(res, at("2026-11-16T15:00:00Z"));
    // Monday 29 March is in summer time (UTC+2): 16:00 local is 14:00 UTC.
    assert_eq!(due(&app, &dst).await.0, at("2027-03-29T14:00:00Z"));

    // Switching the calendar off restores plain elapsed time.
    let (status, _) = calendar(&app, false, office_hours()).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(due(&app, &friday).await.0, at("2026-11-13T23:30:00Z"));
    let (_, c) = get(&app, ADMIN, "/helpdesk/calendar").await;
    assert_eq!(c["enabled"], false);
    assert_eq!(
        c["holidays"].as_array().unwrap().len(),
        1,
        "settings are kept"
    );
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn holidays_and_calendar_changes_recompute_open_tickets_only(db: PgPool) {
    let app = setup(db.clone()).await;
    calendar(&app, true, office_hours()).await;
    // Tuesday 10 November 16:00 local, normal priority (8 h): 1 h Tuesday + 7 h Wednesday.
    let open = new_ticket(&app, SUPPORT, "Open").await;
    created_at(&db, &open, "2026-11-10T15:00:00Z").await;
    let closed = new_ticket(&app, SUPPORT, "Closed").await;
    created_at(&db, &closed, "2026-11-10T15:00:00Z").await;
    calendar(&app, true, office_hours()).await;
    let closed_due = {
        let t = ticket(&app, &closed).await;
        let id = closed.clone();
        let d = &t["ticket"];
        let (code, _) = put(
            &app,
            SUPPORT,
            &format!("/tickets/{id}"),
            json!({ "subject": d["subject"], "status": "closed", "priority": d["priority"], "category": d["category"],
                    "language": d["language"], "owner_user_id": d["owner_user_id"], "customer_id": d["customer_id"],
                    "contact_id": d["contact_id"], "contact_name": d["contact_name"], "contact_email": d["contact_email"] }),
        )
        .await;
        assert_eq!(code, StatusCode::OK);
        due(&app, &closed).await.0
    };
    assert_eq!(due(&app, &open).await.0, at("2026-11-11T15:00:00Z"));

    let (status, h) = post(
        &app,
        ADMIN,
        "/helpdesk/calendar/holidays",
        json!({ "day": "2026-11-11T00:00:00.000", "name": "Armistice" }),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{h}");
    assert_eq!(h["day"], "2026-11-11");
    assert_eq!(
        due(&app, &open).await.0,
        at("2026-11-12T15:00:00Z"),
        "Wednesday is skipped"
    );
    assert_eq!(
        due(&app, &closed).await.0,
        closed_due,
        "closed tickets keep their history"
    );
    let (status, _) = post(
        &app,
        ADMIN,
        "/helpdesk/calendar/holidays",
        json!({ "day": "2026-11-11", "name": "Again" }),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);

    let id = h["id"].as_str().unwrap();
    let (status, _) = api(
        &app,
        ADMIN,
        Method::DELETE,
        &format!("/helpdesk/calendar/holidays/{id}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    assert_eq!(due(&app, &open).await.0, at("2026-11-11T15:00:00Z"));
    let (status, _) = api(
        &app,
        ADMIN,
        Method::DELETE,
        &format!("/helpdesk/calendar/holidays/{id}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    // Shorter opening hours move the due time: 09:00-12:00 gives 3 h a day.
    let hours = json!([1, 2, 3, 4, 5]
        .map(|d| json!({ "weekday": d, "start_time": "09:00", "end_time": "12:00" })));
    calendar(&app, true, hours).await;
    // Tuesday 16:00 is after closing: 8 h = Wed 3 h + Thu 3 h + Fri 2 h -> Friday 11:00 local.
    assert_eq!(due(&app, &open).await.0, at("2026-11-13T10:00:00Z"));
    let audit: i64 = sqlx::query_scalar("SELECT count(*) FROM audit.events WHERE action IN ('business_calendar_updated', 'holiday_added', 'holiday_removed')")
        .fetch_one(&db)
        .await
        .unwrap();
    assert!(audit >= 5);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn pauses_that_span_closed_hours_count_only_business_time(db: PgPool) {
    let app = setup(db.clone()).await;
    calendar(&app, true, office_hours()).await;
    // Friday 13 November 10:00 local (09:00 UTC), normal priority: 8 h ends Monday 10:00 local.
    let id = new_ticket(&app, SUPPORT, "Waited over the weekend").await;
    created_at(&db, &id, "2026-11-13T09:00:00Z").await;
    calendar(&app, true, office_hours()).await;
    assert_eq!(due(&app, &id).await.0, at("2026-11-16T09:00:00Z"));
    // The customer was asked on Friday 16:00 local and answered Monday 10:00 local: 2 business
    // hours. (Recorded as the pause interval the clock keeps.)
    sqlx::query(
        "INSERT INTO helpdesk.sla_pauses (id, company_id, ticket_id, started_at, ended_at)
         VALUES (gen_random_uuid(), $2::uuid, $1::uuid, '2026-11-13T15:00:00Z', '2026-11-16T09:00:00Z')",
    )
    .bind(&id)
    .bind(COMPANY)
    .execute(&db)
    .await
    .unwrap();
    sqlx::query("UPDATE helpdesk.tickets SET sla_paused_seconds = 66 * 3600 WHERE id = $1::uuid")
        .bind(&id)
        .execute(&db)
        .await
        .unwrap();
    calendar(&app, true, office_hours()).await;
    assert_eq!(
        due(&app, &id).await.0,
        at("2026-11-16T11:00:00Z"),
        "10:00 + 2 business hours"
    );
    // Elapsed time would add the whole 66 hours.
    calendar(&app, false, office_hours()).await;
    let t = ticket(&app, &id).await;
    assert_eq!(
        due(&app, &id).await.0,
        when(&t["ticket"]["created_at"]) + chrono::Duration::hours(8 + 66)
    );
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn a_pause_is_recorded_as_an_interval_and_released_on_reopening(db: PgPool) {
    let app = setup(db.clone()).await;
    let id = new_ticket(&app, SUPPORT, "Waiting").await;
    let d = ticket(&app, &id).await;
    let t = &d["ticket"];
    let body = |status: &str| {
        json!({ "subject": t["subject"], "status": status, "priority": t["priority"], "category": t["category"],
                "language": t["language"], "owner_user_id": t["owner_user_id"], "customer_id": t["customer_id"],
                "contact_id": t["contact_id"], "contact_name": t["contact_name"], "contact_email": t["contact_email"] })
    };
    put(
        &app,
        SUPPORT,
        &format!("/tickets/{id}"),
        body("pending_customer"),
    )
    .await;
    let open: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM helpdesk.sla_pauses WHERE ticket_id = $1::uuid AND ended_at IS NULL",
    )
    .bind(&id)
    .fetch_one(&db)
    .await
    .unwrap();
    assert_eq!(open, 1);
    // A second save with the same status does not start another pause.
    put(
        &app,
        SUPPORT,
        &format!("/tickets/{id}"),
        body("pending_customer"),
    )
    .await;
    put(&app, SUPPORT, &format!("/tickets/{id}"), body("open")).await;
    let rows: Vec<(bool,)> = sqlx::query_as(
        "SELECT ended_at IS NOT NULL FROM helpdesk.sla_pauses WHERE ticket_id = $1::uuid",
    )
    .bind(&id)
    .fetch_all(&db)
    .await
    .unwrap();
    assert_eq!(rows, vec![(true,)]);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn at_risk_and_breached_filters_agree_with_the_ticket_in_business_mode(db: PgPool) {
    let app = setup(db.clone()).await;
    // A calendar that is open every day all day (a minute short at midnight).
    let all_week = json!((1..=7)
        .map(|d| json!({ "weekday": d, "start_time": "00:00", "end_time": "23:59" }))
        .collect::<Vec<_>>());
    calendar(&app, true, all_week).await;
    let risky = new_ticket(&app, SUPPORT, "Risky").await;
    let late = new_ticket(&app, SUPPORT, "Late").await;
    let fine = new_ticket(&app, SUPPORT, "Fine").await;
    for id in [&risky, &late] {
        set_priority(&app, id, "urgent").await; // 60 minutes to the first response
    }
    // Created 50 minutes and 3 hours ago.
    sqlx::query("UPDATE helpdesk.tickets SET created_at = now() - interval '50 minutes' WHERE id = $1::uuid").bind(&risky).execute(&db).await.unwrap();
    sqlx::query(
        "UPDATE helpdesk.tickets SET created_at = now() - interval '3 hours' WHERE id = $1::uuid",
    )
    .bind(&late)
    .execute(&db)
    .await
    .unwrap();
    let all_week = json!((1..=7)
        .map(|d| json!({ "weekday": d, "start_time": "00:00", "end_time": "23:59" }))
        .collect::<Vec<_>>());
    calendar(&app, true, all_week).await;
    assert_eq!(
        ticket(&app, &risky).await["ticket"]["sla"]["state"],
        "at_risk"
    );
    assert_eq!(
        ticket(&app, &late).await["ticket"]["sla"]["state"],
        "breached"
    );
    assert_eq!(
        ticket(&app, &fine).await["ticket"]["sla"]["state"],
        "on_time"
    );
    let ids = |q: &'static str| {
        let app = app.clone();
        async move {
            let (_, p) = get(&app, SUPPORT, &format!("/tickets?sla={q}")).await;
            p["items"]
                .as_array()
                .unwrap()
                .iter()
                .map(|t| t["id"].as_str().unwrap().to_string())
                .collect::<Vec<_>>()
        }
    };
    assert_eq!(ids("at_risk").await, vec![risky.clone()]);
    assert_eq!(ids("breached").await, vec![late.clone()]);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn calendar_settings_are_validated_and_for_administrators(db: PgPool) {
    let app = setup(db).await;
    let (status, c) = get(&app, ADMIN, "/helpdesk/calendar").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        (c["enabled"].as_bool(), c["time_zone"].as_str()),
        (Some(false), Some("Europe/Brussels"))
    );
    assert_eq!(
        c["hours"].as_array().unwrap().len(),
        5,
        "a default week is offered"
    );
    for who in [SUPPORT, SALES, VIEWER, TECH] {
        assert_eq!(
            get(&app, who, "/helpdesk/calendar").await.0,
            StatusCode::FORBIDDEN,
            "{who}"
        );
        let (status, _) = calendar_as(&app, who).await;
        assert_eq!(status, StatusCode::FORBIDDEN, "{who}");
        let (status, _) = post(
            &app,
            who,
            "/helpdesk/calendar/holidays",
            json!({ "day": "2026-12-25", "name": "Christmas" }),
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN);
    }
    for body in [
        json!({ "enabled": true, "time_zone": "Mars/Olympus", "hours": office_hours() }),
        json!({ "enabled": true, "time_zone": "Europe/Brussels", "hours": [] }),
        json!({ "enabled": false, "time_zone": "Europe/Brussels", "hours": [{ "weekday": 8, "start_time": "09:00", "end_time": "17:00" }] }),
        json!({ "enabled": false, "time_zone": "Europe/Brussels", "hours": [{ "weekday": 1, "start_time": "17:00", "end_time": "09:00" }] }),
        json!({ "enabled": false, "time_zone": "Europe/Brussels", "hours": [{ "weekday": 1, "start_time": "nine", "end_time": "17:00" }] }),
        json!({ "enabled": false, "time_zone": "Europe/Brussels", "hours": [
            { "weekday": 1, "start_time": "09:00", "end_time": "17:00" }, { "weekday": 1, "start_time": "09:00", "end_time": "12:00" }] }),
    ] {
        let (status, r) = put(&app, ADMIN, "/helpdesk/calendar", body.clone()).await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{body} {r}");
    }
    let (status, _) = post(
        &app,
        ADMIN,
        "/helpdesk/calendar/holidays",
        json!({ "day": "2026-12-25", "name": " " }),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
}

async fn calendar_as(app: &axum::Router, user: &str) -> (StatusCode, Value) {
    put(
        app,
        user,
        "/helpdesk/calendar",
        json!({ "enabled": true, "time_zone": "Europe/Brussels", "hours": office_hours() }),
    )
    .await
}

// ---------------------------------------------------------------------------------------------
// Automation rules
// ---------------------------------------------------------------------------------------------

async fn rule(app: &axum::Router, body: Value) -> Value {
    let (status, r) = post(app, ADMIN, "/helpdesk/automation/rules", body).await;
    assert_eq!(status, StatusCode::CREATED, "{r}");
    r
}

async fn applied(db: &PgPool, ticket: &str) -> Vec<(String, String, Option<String>)> {
    sqlx::query_as(
        "SELECT data->>'rule_name', data->>'change', data->>'to' FROM audit.events
         WHERE entity_id = $1::uuid AND action = 'automation_rule_applied' ORDER BY id",
    )
    .bind(ticket)
    .fetch_all(db)
    .await
    .unwrap()
}

async fn team(app: &axum::Router, name: &str) -> String {
    let (status, t) = post(
        app,
        ADMIN,
        "/helpdesk/teams",
        json!({ "name": name, "members": [] }),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{t}");
    t["id"].as_str().unwrap().to_string()
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn rules_run_in_order_on_creation_and_every_change_is_audited_with_the_rule(db: PgPool) {
    let app = setup(db.clone()).await;
    let sound = team(&app, "Sound team").await;
    rule(&app, json!({ "name": "Noise", "trigger": "created",
        "conditions": { "subject_contains": "noise" },
        "actions": [{ "kind": "set_priority", "value": "high" }, { "kind": "add_tag", "value": "Acoustic" },
                    { "kind": "set_team", "team_id": sound }] }))
    .await;
    // Sees the priority the first rule set.
    rule(&app, json!({ "name": "High is escalated", "trigger": "created",
        "conditions": { "priorities": ["high"], "team_id": sound },
        "actions": [{ "kind": "add_tag", "value": "escalated" }, { "kind": "set_owner", "user_id": SUPPORT_ID }] }))
    .await;
    rule(
        &app,
        json!({ "name": "Leads", "trigger": "created", "conditions": { "categories": ["lead"] },
        "actions": [{ "kind": "set_priority", "value": "low" }] }),
    )
    .await;
    let (_, rules) = get(&app, ADMIN, "/helpdesk/automation/rules").await;
    assert_eq!(
        rules
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r["position"].as_i64().unwrap())
            .collect::<Vec<_>>(),
        vec![1, 2, 3]
    );

    let id = new_ticket(&app, SALES, "Pump makes NOISE").await;
    let t = ticket(&app, &id).await;
    assert_eq!(t["ticket"]["priority"], "high");
    assert_eq!(t["ticket"]["team_id"], sound.as_str());
    assert_eq!(t["ticket"]["owner_user_id"], SUPPORT_ID);
    assert_eq!(t["ticket"]["tags"], json!(["acoustic", "escalated"]));
    // SLA follows the new priority (high: 4 h first response).
    let created = when(&t["ticket"]["created_at"]);
    assert_eq!(
        when(&t["ticket"]["sla"]["first_response"]["due_at"]),
        created + chrono::Duration::minutes(240)
    );
    let log = applied(&db, &id).await;
    assert_eq!(
        log.iter()
            .map(|(r, c, _)| (r.as_str(), c.as_str()))
            .collect::<Vec<_>>(),
        vec![
            ("Noise", "set_priority"),
            ("Noise", "add_tag"),
            ("Noise", "set_team"),
            ("High is escalated", "add_tag"),
            ("High is escalated", "set_owner"),
        ]
    );
    // A ticket that matches nothing is left alone and nothing is audited.
    let other = new_ticket(&app, SALES, "Quotation question").await;
    assert_eq!(ticket(&app, &other).await["ticket"]["priority"], "normal");
    assert!(applied(&db, &other).await.is_empty());
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn ordering_decides_which_rule_wins_and_a_stop_rule_ends_the_run(db: PgPool) {
    let app = setup(db.clone()).await;
    let a = rule(
        &app,
        json!({ "name": "A low", "trigger": "created", "conditions": {},
        "actions": [{ "kind": "set_priority", "value": "low" }] }),
    )
    .await;
    let b = rule(
        &app,
        json!({ "name": "B urgent", "trigger": "created", "conditions": {},
        "actions": [{ "kind": "set_priority", "value": "urgent" }] }),
    )
    .await;
    let (a_id, b_id) = (a["id"].as_str().unwrap(), b["id"].as_str().unwrap());
    // All matching rules apply in order: the later one has the last word.
    let t1 = new_ticket(&app, SALES, "One").await;
    assert_eq!(ticket(&app, &t1).await["ticket"]["priority"], "urgent");
    // Reorder: now A runs last.
    let (status, r) = put(
        &app,
        ADMIN,
        "/helpdesk/automation/order",
        json!({ "ids": [b_id, a_id] }),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{r}");
    assert_eq!(r[0]["name"], "B urgent");
    let t2 = new_ticket(&app, SALES, "Two").await;
    assert_eq!(ticket(&app, &t2).await["ticket"]["priority"], "low");
    // B stops the run: A is not applied after it.
    let mut stop = b.clone();
    stop["stop_on_match"] = json!(true);
    let (status, r) = put(&app, ADMIN, &format!("/helpdesk/automation/rules/{b_id}"), json!({
        "name": "B urgent", "trigger": "created", "conditions": {}, "actions": b["actions"], "stop_on_match": true, "active": true })).await;
    assert_eq!(status, StatusCode::OK, "{r} {stop}");
    let t3 = new_ticket(&app, SALES, "Three").await;
    assert_eq!(ticket(&app, &t3).await["ticket"]["priority"], "urgent");
    assert_eq!(applied(&db, &t3).await.len(), 1);
    // A disabled rule is skipped.
    let (status, _) = put(&app, ADMIN, &format!("/helpdesk/automation/rules/{b_id}"), json!({
        "name": "B urgent", "trigger": "created", "conditions": {}, "actions": b["actions"], "active": false })).await;
    assert_eq!(status, StatusCode::OK);
    let t4 = new_ticket(&app, SALES, "Four").await;
    assert_eq!(ticket(&app, &t4).await["ticket"]["priority"], "low");
    // The order must list every rule once.
    for ids in [
        json!([a_id]),
        json!([a_id, a_id]),
        json!([a_id, b_id, Uuid::now_v7().to_string()]),
    ] {
        let (status, _) = put(
            &app,
            ADMIN,
            "/helpdesk/automation/order",
            json!({ "ids": ids }),
        )
        .await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    }
    // Delete.
    let (status, _) = api(
        &app,
        ADMIN,
        Method::DELETE,
        &format!("/helpdesk/automation/rules/{a_id}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    let (_, rules) = get(&app, ADMIN, "/helpdesk/automation/rules").await;
    assert_eq!(rules.as_array().unwrap().len(), 1);
    let (status, _) = api(
        &app,
        ADMIN,
        Method::DELETE,
        &format!("/helpdesk/automation/rules/{a_id}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn rules_on_update_cannot_loop(db: PgPool) {
    let app = setup(db.clone()).await;
    // Each rule undoes the other. Every event runs each rule once and the changes do not raise
    // new events, so this ends.
    rule(
        &app,
        json!({ "name": "Up", "trigger": "updated", "conditions": { "priorities": ["normal"] },
        "actions": [{ "kind": "set_priority", "value": "high" }] }),
    )
    .await;
    rule(
        &app,
        json!({ "name": "Down", "trigger": "updated", "conditions": { "priorities": ["high"] },
        "actions": [{ "kind": "set_priority", "value": "normal" }] }),
    )
    .await;
    let id = new_ticket(&app, SALES, "Flapping").await;
    assert!(
        applied(&db, &id).await.is_empty(),
        "the rules are for updates"
    );
    let r = set_priority(&app, &id, "normal").await;
    assert_eq!(r["ticket"]["priority"], "normal");
    assert_eq!(applied(&db, &id).await.len(), 2, "each rule ran once");
    let r = set_priority(&app, &id, "normal").await;
    assert_eq!(r["ticket"]["priority"], "normal");
    assert_eq!(
        applied(&db, &id).await.len(),
        4,
        "once more per event, never within one"
    );
}

fn mail(from: &str, subject: &str, body: &str) -> Vec<u8> {
    format!("From: {from}\r\nTo: sales@example.com\r\nSubject: {subject}\r\nContent-Type: text/plain; charset=utf-8\r\n\r\n{body}\r\n").into_bytes()
}

async fn import(app: &axum::Router, raw: Vec<u8>) -> Value {
    let (status, r) = call_raw(
        app,
        SUPPORT,
        "/api/v1/helpdesk/inbound",
        "message/rfc822",
        raw,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{r}");
    r
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn rules_apply_to_incoming_mail_for_new_tickets_and_customer_replies(db: PgPool) {
    let app = setup(db.clone()).await;
    rule(&app, json!({ "name": "Alpha mail", "trigger": "created",
        "conditions": { "channels": ["email"], "sender_domain": "alpha.example", "owner": "empty" },
        "actions": [{ "kind": "add_tag", "value": "alpha" }, { "kind": "set_category", "value": "service" }] })).await;
    rule(&app, json!({ "name": "Reply bumps", "trigger": "customer_replied", "conditions": { "tag": "alpha" },
        "actions": [{ "kind": "set_priority", "value": "urgent" }] })).await;
    // Only a reply from the customer, not creation, triggers the second rule.
    let r = import(
        &app,
        mail(
            "jan.peeters@alpha.example",
            "Leak at the pump",
            "Water everywhere",
        ),
    )
    .await;
    assert_eq!(r["new_ticket"], true);
    let id = r["ticket_id"].as_str().unwrap().to_string();
    let number = r["ticket_number"].as_str().unwrap().to_string();
    let t = ticket(&app, &id).await;
    assert_eq!(t["ticket"]["tags"], json!(["alpha"]));
    assert_eq!(t["ticket"]["category"], "service");
    assert_eq!(t["ticket"]["priority"], "normal");
    // Rules run in the ingest transaction: the audit event has no user (it is the system).
    let by_system: i64 = sqlx::query_scalar("SELECT count(*) FROM audit.events WHERE entity_id = $1::uuid AND action = 'automation_rule_applied' AND user_id IS NULL")
        .bind(&id)
        .fetch_one(&db)
        .await
        .unwrap();
    assert_eq!(by_system, 2);
    // Other senders do not match.
    let r = import(&app, mail("someone@elsewhere.example", "Hello", "Hi")).await;
    let other = ticket(&app, r["ticket_id"].as_str().unwrap()).await;
    assert_eq!(other["ticket"]["tags"], json!([]));

    // The customer answers: the ticket is found by its number and the rule fires.
    let r = import(
        &app,
        mail(
            "jan.peeters@alpha.example",
            &format!("Re: Leak [Ticket#{number}]"),
            "Still leaking",
        ),
    )
    .await;
    assert_eq!(
        (r["new_ticket"].as_bool(), r["ticket_id"].as_str()),
        (Some(false), Some(id.as_str()))
    );
    let t = ticket(&app, &id).await;
    assert_eq!(t["ticket"]["priority"], "urgent");
    assert_eq!(applied(&db, &id).await.last().unwrap().0, "Reply bumps");
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn a_dry_run_shows_the_matches_and_changes_nothing(db: PgPool) {
    let app = setup(db.clone()).await;
    rule(&app, json!({ "name": "Noise", "trigger": "created", "conditions": { "subject_contains": "noise" },
        "actions": [{ "kind": "set_priority", "value": "urgent" }, { "kind": "add_tag", "value": "loud" }] })).await;
    rule(&app, json!({ "name": "Never", "trigger": "created", "conditions": { "subject_contains": "zebra" },
        "actions": [{ "kind": "set_priority", "value": "low" }] })).await;
    rule(
        &app,
        json!({ "name": "On update", "trigger": "updated", "conditions": {},
        "actions": [{ "kind": "set_priority", "value": "low" }] }),
    )
    .await;
    // Created before the rules existed? No: create the ticket first with no rule matching "pump".
    let id = new_ticket(&app, SALES, "Pump noise at night").await;
    // The ticket was created with the rules in place, so reset what they did.
    sqlx::query("UPDATE helpdesk.tickets SET priority = 'normal' WHERE id = $1::uuid")
        .bind(&id)
        .execute(&db)
        .await
        .unwrap();
    sqlx::query("DELETE FROM helpdesk.ticket_tags WHERE ticket_id = $1::uuid")
        .bind(&id)
        .execute(&db)
        .await
        .unwrap();
    sqlx::query("DELETE FROM audit.events WHERE entity_id = $1::uuid AND action = 'automation_rule_applied'").bind(&id).execute(&db).await.unwrap();
    let before = ticket(&app, &id).await;

    let (status, d) = post(
        &app,
        ADMIN,
        "/helpdesk/automation/dry-run",
        json!({ "ticket_id": id, "trigger": "created" }),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{d}");
    let results: Vec<(&str, &str)> = d["rules"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            (
                r["rule_name"].as_str().unwrap(),
                r["result"].as_str().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        results,
        vec![
            ("Noise", "matched"),
            ("Never", "no_match"),
            ("On update", "other_trigger")
        ]
    );
    assert_eq!(d["rules"][0]["changes"].as_array().unwrap().len(), 2);
    assert_eq!(d["rules"][0]["changes"][0]["to"], "urgent");
    assert_eq!(d["ticket_number"], before["ticket"]["number"]);

    let after = ticket(&app, &id).await;
    assert_eq!(after["ticket"]["priority"], before["ticket"]["priority"]);
    assert_eq!(after["ticket"]["tags"], before["ticket"]["tags"]);
    assert_eq!(
        after["ticket"]["updated_at"],
        before["ticket"]["updated_at"]
    );
    assert!(applied(&db, &id).await.is_empty());
    let (status, _) = post(
        &app,
        ADMIN,
        "/helpdesk/automation/dry-run",
        json!({ "ticket_id": Uuid::now_v7(), "trigger": "created" }),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    let (status, _) = post(
        &app,
        ADMIN,
        "/helpdesk/automation/dry-run",
        json!({ "ticket_id": id, "trigger": "sometimes" }),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn rules_are_validated_for_administrators_and_isolated_per_company(db: PgPool) {
    let app = setup(db.clone()).await;
    second_company(&db).await;
    for who in [SUPPORT, SALES, VIEWER, TECH] {
        assert_eq!(
            get(&app, who, "/helpdesk/automation/rules").await.0,
            StatusCode::FORBIDDEN,
            "{who}"
        );
        let (status, _) = post(&app, who, "/helpdesk/automation/rules", json!({ "name": "x", "trigger": "created", "actions": [{ "kind": "add_tag", "value": "x" }] })).await;
        assert_eq!(status, StatusCode::FORBIDDEN);
    }
    let foreign_team = {
        let (_, t) = post(
            &app,
            OTHER,
            "/helpdesk/teams",
            json!({ "name": "Theirs", "members": [] }),
        )
        .await;
        t["id"].as_str().unwrap().to_string()
    };
    for (what, body) in [
        (
            "no name",
            json!({ "name": " ", "trigger": "created", "actions": [{ "kind": "add_tag", "value": "x" }] }),
        ),
        (
            "no actions",
            json!({ "name": "r", "trigger": "created", "actions": [] }),
        ),
        (
            "bad trigger",
            json!({ "name": "r", "trigger": "hourly", "actions": [{ "kind": "add_tag", "value": "x" }] }),
        ),
        (
            "bad priority",
            json!({ "name": "r", "trigger": "created", "actions": [{ "kind": "set_priority", "value": "huge" }] }),
        ),
        (
            "bad kind",
            json!({ "name": "r", "trigger": "created", "actions": [{ "kind": "send_mail" }] }),
        ),
        (
            "bad condition",
            json!({ "name": "r", "trigger": "created", "conditions": { "channels": ["fax"] }, "actions": [{ "kind": "add_tag", "value": "x" }] }),
        ),
        (
            "bad owner flag",
            json!({ "name": "r", "trigger": "created", "conditions": { "owner": "maybe" }, "actions": [{ "kind": "add_tag", "value": "x" }] }),
        ),
        (
            "owner of another company",
            json!({ "name": "r", "trigger": "created", "actions": [{ "kind": "set_owner", "user_id": Uuid::now_v7() }] }),
        ),
        (
            "team of another company",
            json!({ "name": "r", "trigger": "created", "actions": [{ "kind": "set_team", "team_id": foreign_team }] }),
        ),
        (
            "long tag",
            json!({ "name": "r", "trigger": "created", "actions": [{ "kind": "add_tag", "value": "x".repeat(41) }] }),
        ),
        (
            "internal without flag",
            json!({ "name": "r", "trigger": "created", "actions": [{ "kind": "set_internal" }] }),
        ),
    ] {
        let (status, r) = post(&app, ADMIN, "/helpdesk/automation/rules", body).await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{what}: {r}");
    }
    let mine = rule(&app, json!({ "name": "Mine", "trigger": "created", "conditions": {}, "actions": [{ "kind": "add_tag", "value": "mine" }] })).await;
    let mine_id = mine["id"].as_str().unwrap();
    // The other company sees none, cannot change or delete it, and its tickets are not touched.
    let (_, theirs) = get(&app, OTHER, "/helpdesk/automation/rules").await;
    assert_eq!(theirs, json!([]));
    assert_eq!(
        put(
            &app,
            OTHER,
            &format!("/helpdesk/automation/rules/{mine_id}"),
            json!({
        "name": "x", "trigger": "created", "actions": [{ "kind": "add_tag", "value": "x" }] })
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        api(
            &app,
            OTHER,
            Method::DELETE,
            &format!("/helpdesk/automation/rules/{mine_id}"),
            None
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    let (status, t) = post(
        &app,
        OTHER,
        "/tickets",
        json!({ "subject": "Their ticket", "contact_email": "x@other.example", "body": "hi" }),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{t}");
    assert_eq!(t["ticket"]["tags"], json!([]));
    let mine_ticket = new_ticket(&app, SALES, "Ours").await;
    assert_eq!(
        ticket(&app, &mine_ticket).await["ticket"]["tags"],
        json!(["mine"])
    );
    let (status, _) = post(
        &app,
        OTHER,
        "/helpdesk/automation/dry-run",
        json!({ "ticket_id": mine_ticket, "trigger": "created" }),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    let (status, _) = put(
        &app,
        OTHER,
        "/helpdesk/automation/order",
        json!({ "ids": [mine_id] }),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
}

// ---------------------------------------------------------------------------------------------
// Split
// ---------------------------------------------------------------------------------------------

async fn note(app: &axum::Router, id: &str, body: &str, attachments: Vec<String>) -> Value {
    let (status, t) = post(
        app,
        SUPPORT,
        &format!("/tickets/{id}/notes"),
        json!({ "body": body, "attachment_ids": attachments }),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{t}");
    t
}

async fn upload_pdf(app: &axum::Router, id: &str, name: &str) -> String {
    let (status, _, body) = raw(
        app,
        Who::Staff(SUPPORT),
        Method::POST,
        &format!("/api/v1/tickets/{id}/attachments?filename={name}"),
        Some(("application/octet-stream", PDF.to_vec())),
    )
    .await;
    assert_eq!(
        status,
        StatusCode::CREATED,
        "{}",
        String::from_utf8_lossy(&body)
    );
    serde_json::from_slice::<Value>(&body).unwrap()["id"]
        .as_str()
        .unwrap()
        .to_string()
}

fn article_ids(t: &Value) -> Vec<String> {
    t["articles"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| a["id"].as_str().unwrap().to_string())
        .collect()
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn splitting_moves_messages_and_their_files_to_a_new_ticket(db: PgPool) {
    let app = setup(db.clone()).await;
    let id = new_ticket(&app, SUPPORT, "Two problems in one mail").await;
    let file = upload_pdf(&app, &id, "invoice.pdf").await;
    note(&app, &id, "Second topic: the invoice", vec![file.clone()]).await;
    let t = note(&app, &id, "Back to the first topic", vec![]).await;
    let ids = article_ids(&t);
    assert_eq!(ids.len(), 3);
    let moved = &ids[1];

    // Permissions and bad input first.
    for who in [VIEWER, TECH] {
        let (status, _) = post(
            &app,
            who,
            &format!("/tickets/{id}/split"),
            json!({ "article_ids": [moved] }),
        )
        .await;
        assert!(status == StatusCode::FORBIDDEN, "{who} {status}");
    }
    let (status, _) = post(
        &app,
        SUPPORT,
        &format!("/tickets/{id}/split"),
        json!({ "article_ids": [] }),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    let (status, _) = post(
        &app,
        SUPPORT,
        &format!("/tickets/{id}/split"),
        json!({ "article_ids": [Uuid::now_v7()] }),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    let (status, r) = post(
        &app,
        SUPPORT,
        &format!("/tickets/{id}/split"),
        json!({ "article_ids": ids }),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT, "everything would move: {r}");
    let (status, _) = post(
        &app,
        SUPPORT,
        &format!("/tickets/{}/split", Uuid::now_v7()),
        json!({ "article_ids": [moved] }),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    let (status, n) = post(
        &app,
        SUPPORT,
        &format!("/tickets/{id}/split"),
        json!({ "article_ids": [moved], "subject": "Invoice question" }),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{n}");
    let nt = &n["ticket"];
    assert_eq!(nt["subject"], "Invoice question");
    assert_eq!(nt["status"], "open");
    assert_ne!(nt["id"], id.as_str());
    assert_ne!(nt["number"], t["ticket"]["number"]);
    // Same customer and contact data; own service level clock starting now.
    for f in ["customer_id", "contact_email", "language"] {
        assert_eq!(nt[f], t["ticket"][f], "{f}");
    }
    assert!(when(&nt["created_at"]) > when(&t["ticket"]["created_at"]));
    assert_eq!(nt["sla"]["state"], "on_time");
    assert_eq!(
        when(&nt["sla"]["first_response"]["due_at"]),
        when(&nt["created_at"]) + chrono::Duration::minutes(480)
    );
    // The moved message (with its file) and a system note linking back to the source.
    let new_articles = n["articles"].as_array().unwrap();
    assert_eq!(new_articles.len(), 2);
    let with_file = new_articles
        .iter()
        .find(|a| a["id"].as_str() == Some(moved))
        .unwrap();
    assert_eq!(with_file["attachments"][0]["id"], file.as_str());
    assert!(new_articles.iter().any(|a| a["body"]
        .as_str()
        .unwrap()
        .contains(t["ticket"]["number"].as_str().unwrap())));
    // The source keeps the others plus a note pointing at the new ticket.
    let source = ticket(&app, &id).await;
    assert_eq!(source["articles"].as_array().unwrap().len(), 3);
    assert!(source["articles"]
        .as_array()
        .unwrap()
        .iter()
        .all(|a| a["id"].as_str() != Some(moved)));
    assert!(source["articles"]
        .as_array()
        .unwrap()
        .iter()
        .any(|a| a["body"]
            .as_str()
            .unwrap()
            .contains(nt["number"].as_str().unwrap())));
    let on_source: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM helpdesk.ticket_attachments WHERE ticket_id = $1::uuid",
    )
    .bind(&id)
    .fetch_one(&db)
    .await
    .unwrap();
    assert_eq!(on_source, 0, "the file left with its message");
    // Audit on both tickets.
    for ticket_id in [id.as_str(), nt["id"].as_str().unwrap()] {
        let n: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM audit.events WHERE entity_id = $1::uuid AND action = 'split'",
        )
        .bind(ticket_id)
        .fetch_one(&db)
        .await
        .unwrap();
        assert_eq!(n, 1, "{ticket_id}");
    }
    // A split cannot be repeated with a message that is no longer there.
    let (status, _) = post(
        &app,
        SUPPORT,
        &format!("/tickets/{id}/split"),
        json!({ "article_ids": [moved] }),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    // Another company cannot split it.
    second_company(&db).await;
    let (status, _) = post(
        &app,
        OTHER,
        &format!("/tickets/{id}/split"),
        json!({ "article_ids": [ids[2]] }),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    // The new ticket is a normal ticket: rules for new tickets run on it.
    rule(&app, json!({ "name": "Tag splits", "trigger": "created", "conditions": {}, "actions": [{ "kind": "add_tag", "value": "from-split" }] })).await;
    let (status, n2) = post(
        &app,
        SUPPORT,
        &format!("/tickets/{id}/split"),
        json!({ "article_ids": [ids[2]] }),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(n2["ticket"]["tags"], json!(["from-split"]));
    assert_eq!(
        n2["ticket"]["subject"], "Two problems in one mail",
        "falls back to the source subject"
    );
}

// ---------------------------------------------------------------------------------------------
// Reports
// ---------------------------------------------------------------------------------------------

struct Fixture {
    t1: String,
    t2: String,
    a: String,
    c: String,
    /// The Monday of the fixture: tickets A and B were created on it, C and D the next day.
    monday: chrono::NaiveDate,
}

impl Fixture {
    fn day(&self, offset: i64) -> String {
        (self.monday + chrono::Duration::days(offset)).to_string()
    }
    /// `from` Sunday before to Saturday after.
    fn range(&self) -> String {
        format!("from={}&to={}", self.day(-1), self.day(5))
    }
}

/// The most recent Monday that is at least three days ago, so the fixture is always in the past.
fn fixture_monday() -> chrono::NaiveDate {
    use chrono::Datelike;
    let d = Utc::now().date_naive() - chrono::Duration::days(3);
    d - chrono::Duration::days(i64::from(d.weekday().num_days_from_monday()))
}

/// Four tickets with known times, teams, tags, time entries and ratings.
#[allow(clippy::type_complexity)]
async fn fixture(app: &axum::Router, db: &PgPool) -> Fixture {
    let t1 = team(app, "Alpha team").await;
    let t2 = team(app, "Beta team").await;
    let monday = fixture_monday();
    let ts =
        |offset: i64, time: &str| format!("{}T{time}Z", monday + chrono::Duration::days(offset));
    let a = new_ticket(app, SUPPORT, "A").await;
    let b = new_ticket(app, SUPPORT, "B").await;
    let c = new_ticket(app, SUPPORT, "C").await;
    let d = new_ticket(app, SUPPORT, "D").await;
    sqlx::query("UPDATE helpdesk.tickets SET category = 'complaint' WHERE id = $1::uuid")
        .bind(&c)
        .execute(db)
        .await
        .unwrap();
    // (ticket, team, created, first response after minutes, closed after minutes, FR due after minutes)
    let rows: [(
        &String,
        Option<&String>,
        String,
        Option<i32>,
        Option<i32>,
        i32,
    ); 4] = [
        (&a, Some(&t1), ts(0, "09:00:00"), Some(30), Some(240), 480),
        (&b, Some(&t1), ts(0, "10:00:00"), Some(90), Some(600), 60),
        (&c, Some(&t2), ts(1, "10:00:00"), None, None, 480),
        (&d, None, ts(1, "11:00:00"), None, None, 60 * 24 * 365 * 5),
    ];
    for (id, tm, created, fr, closed, fr_due) in rows {
        sqlx::query(
            "UPDATE helpdesk.tickets SET team_id = $2, created_at = $3::timestamptz,
                    first_response_at = CASE WHEN $4::int IS NULL THEN NULL ELSE $3::timestamptz + $4 * interval '1 minute' END,
                    closed_at = CASE WHEN $5::int IS NULL THEN NULL ELSE $3::timestamptz + $5 * interval '1 minute' END,
                    status = CASE WHEN $5::int IS NULL THEN 'open' ELSE 'closed' END,
                    first_response_due_at = $3::timestamptz + $6 * interval '1 minute',
                    resolution_due_at = $3::timestamptz + $6 * interval '9 minutes',
                    sla_first_response_minutes = $6, sla_resolution_minutes = 4320
             WHERE id = $1::uuid",
        )
        .bind(id)
        .bind(tm.map(|t| Uuid::parse_str(t).unwrap()))
        .bind(&created)
        .bind(fr)
        .bind(closed)
        .bind(fr_due)
        .execute(db)
        .await
        .unwrap();
    }
    for (id, tag) in [(&a, "warranty"), (&b, "warranty"), (&c, "urgent")] {
        sqlx::query("INSERT INTO helpdesk.ticket_tags (ticket_id, tag) VALUES ($1::uuid, $2)")
            .bind(id)
            .bind(tag)
            .execute(db)
            .await
            .unwrap();
    }
    sqlx::query("INSERT INTO helpdesk.ticket_tags (ticket_id, tag) VALUES ($1::uuid, '=cmd')")
        .bind(&a)
        .execute(db)
        .await
        .unwrap();
    // Time: support 20 billable, admin 30 not billable, both on 2 November.
    let (s, _) = post(
        app,
        SUPPORT,
        &format!("/tickets/{a}/time-entries"),
        json!({ "minutes": 20, "billable": true }),
    )
    .await;
    assert_eq!(s, StatusCode::CREATED);
    let (s, _) = post(
        app,
        ADMIN,
        &format!("/tickets/{a}/time-entries"),
        json!({ "minutes": 30, "billable": false }),
    )
    .await;
    assert_eq!(s, StatusCode::CREATED);
    sqlx::query("UPDATE helpdesk.time_entries SET created_at = $1::timestamptz")
        .bind(ts(0, "12:00:00"))
        .execute(db)
        .await
        .unwrap();
    // Ratings: one good (A) and one bad (B) on 2 November.
    for (id, rating) in [(&a, "good"), (&b, "bad")] {
        sqlx::query(
            "INSERT INTO helpdesk.csat_surveys (id, company_id, ticket_id, token_hash, expires_at, rating, rated_at)
             VALUES (gen_random_uuid(), $3::uuid, $1::uuid, $4, now() + interval '30 days', $2, $5::timestamptz)",
        )
        .bind(id)
        .bind(rating)
        .bind(COMPANY)
        .bind(Uuid::now_v7().as_bytes().to_vec())
        .bind(ts(0, "15:00:00"))
        .execute(db)
        .await
        .unwrap();
    }
    Fixture {
        t1,
        t2,
        a,
        c,
        monday,
    }
}

fn num(v: &Value) -> f64 {
    v.as_f64().unwrap_or_else(|| panic!("not a number: {v}"))
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn the_report_numbers_match_a_known_fixture(db: PgPool) {
    let app = setup(db.clone()).await;
    let f = fixture(&app, &db).await;
    let (status, r) = get(&app, SUPPORT, &format!("/helpdesk/reports?{}", f.range())).await;
    assert_eq!(status, StatusCode::OK, "{r}");
    assert_eq!(r["basis"], "elapsed");
    let volume: Vec<(String, i64, i64)> = r["volume"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| {
            (
                v["period"].as_str().unwrap().into(),
                v["created"].as_i64().unwrap(),
                v["closed"].as_i64().unwrap(),
            )
        })
        .collect();
    assert_eq!(volume.len(), 7);
    assert_eq!(volume[1], (f.day(0), 2, 2));
    assert_eq!(volume[2], (f.day(1), 2, 0));
    assert_eq!(volume.iter().map(|v| v.1).sum::<i64>(), 4);
    // Weeks start on Monday: one bucket for 2 and 3 November, one empty for the week before.
    let (_, w) = get(
        &app,
        SUPPORT,
        &format!("/helpdesk/reports?{}&group=week", f.range()),
    )
    .await;
    let weeks: Vec<(String, i64, i64)> = w["volume"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| {
            (
                v["period"].as_str().unwrap().into(),
                v["created"].as_i64().unwrap(),
                v["closed"].as_i64().unwrap(),
            )
        })
        .collect();
    assert_eq!(weeks, vec![(f.day(-7), 0, 0), (f.day(0), 4, 2)]);

    // Open backlog: C and D (not date limited).
    let counts = |v: &Value| -> Vec<(String, i64)> {
        v.as_array()
            .unwrap()
            .iter()
            .map(|r| {
                (
                    r["label"].as_str().unwrap().to_string(),
                    r["count"].as_i64().unwrap(),
                )
            })
            .collect()
    };
    assert_eq!(counts(&r["backlog_by_status"]), vec![("open".into(), 2)]);
    assert_eq!(
        counts(&r["backlog_by_priority"]),
        vec![("normal".into(), 2)]
    );
    assert_eq!(
        counts(&r["backlog_by_owner"]),
        vec![("Support Agent".into(), 2)]
    );
    let mut by_team = counts(&r["backlog_by_team"]);
    by_team.sort();
    assert_eq!(by_team, vec![("".into(), 1), ("Beta team".into(), 1)]);

    // Times: first response 30 and 90 minutes, resolution 240 and 600.
    assert_eq!(
        (
            r["first_response"]["count"].as_i64(),
            num(&r["first_response"]["average_minutes"]),
            num(&r["first_response"]["p90_minutes"])
        ),
        (Some(2), 60.0, 90.0)
    );
    assert_eq!(
        (
            r["resolution"]["count"].as_i64(),
            num(&r["resolution"]["average_minutes"]),
            num(&r["resolution"]["p90_minutes"])
        ),
        (Some(2), 420.0, 600.0)
    );
    // SLA: B answered after its 60 minutes and C is overdue; A and D are fine.
    assert_eq!(
        (
            r["sla"]["tickets"].as_i64(),
            r["sla"]["breached"].as_i64(),
            r["sla"]["compliant"].as_i64()
        ),
        (Some(4), Some(2), Some(2))
    );
    assert_eq!(num(&r["sla"]["compliance_percent"]), 50.0);
    let teams: Vec<(String, i64, i64)> = r["breaches_by_team"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| {
            (
                t["team"].as_str().unwrap().into(),
                t["tickets"].as_i64().unwrap(),
                t["breached"].as_i64().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        teams,
        vec![
            ("Alpha team".into(), 2, 1),
            ("Beta team".into(), 1, 1),
            ("".into(), 1, 0)
        ]
    );
    assert_eq!(
        (
            r["csat"]["responses"].as_i64(),
            r["csat"]["good"].as_i64(),
            r["csat"]["bad"].as_i64()
        ),
        (Some(2), Some(1), Some(1))
    );
    assert_eq!(num(&r["csat"]["average_percent"]), 50.0);
    assert_eq!(
        counts(&r["top_categories"]),
        vec![("question".into(), 3), ("complaint".into(), 1)]
    );
    assert_eq!(
        counts(&r["top_tags"]),
        vec![
            ("warranty".into(), 2),
            ("=cmd".into(), 1),
            ("urgent".into(), 1)
        ]
    );
    let time: Vec<(String, i64, i64, i64)> = r["time_by_user"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| {
            (
                t["name"].as_str().unwrap().into(),
                t["total_minutes"].as_i64().unwrap(),
                t["billable_minutes"].as_i64().unwrap(),
                t["non_billable_minutes"].as_i64().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        time,
        vec![
            ("Admin User".into(), 30, 0, 30),
            ("Support Agent".into(), 20, 20, 0)
        ]
    );
    assert_eq!(
        (
            r["time_billable_minutes"].as_i64(),
            r["time_non_billable_minutes"].as_i64()
        ),
        (Some(20), Some(30))
    );

    // Filters: a team, a category, an owner.
    let (_, r) = get(
        &app,
        SUPPORT,
        &format!("/helpdesk/reports?{}&team_id={}", f.range(), f.t1),
    )
    .await;
    assert_eq!(
        r["volume"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v["created"].as_i64().unwrap())
            .sum::<i64>(),
        2
    );
    assert_eq!(r["backlog_by_status"], json!([]));
    assert_eq!(
        (r["sla"]["tickets"].as_i64(), r["sla"]["breached"].as_i64()),
        (Some(2), Some(1))
    );
    let (_, r) = get(
        &app,
        SUPPORT,
        &format!("/helpdesk/reports?{}&team_id={}", f.range(), f.t2),
    )
    .await;
    assert_eq!(r["first_response"]["count"], 0);
    assert_eq!(r["first_response"]["average_minutes"], Value::Null);
    let (_, r) = get(
        &app,
        SUPPORT,
        &format!("/helpdesk/reports?{}&category=complaint", f.range()),
    )
    .await;
    assert_eq!(
        r["volume"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v["created"].as_i64().unwrap())
            .sum::<i64>(),
        1
    );
    let (_, r) = get(
        &app,
        SUPPORT,
        &format!("/helpdesk/reports?{}&owner_id={SUPPORT_ID}", f.range()),
    )
    .await;
    assert_eq!(r["sla"]["tickets"], 4);
    let (_, r) = get(
        &app,
        SUPPORT,
        &format!(
            "/helpdesk/reports?{}&owner_id={}",
            f.range(),
            Uuid::now_v7()
        ),
    )
    .await;
    assert_eq!(r["sla"]["tickets"], 0);
    assert_eq!(r["sla"]["compliance_percent"], Value::Null);
    // The generated client sends dates as date-times.
    let (status, dt) = get(
        &app,
        SUPPORT,
        &format!(
            "/helpdesk/reports?from={}T00:00:00.000&to={}T00:00:00.000",
            f.day(-1),
            f.day(5)
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{dt}");
    let (_, plain) = get(&app, SUPPORT, &format!("/helpdesk/reports?{}", f.range())).await;
    assert_eq!(dt["volume"], plain["volume"]);
    assert_eq!(dt["sla"], plain["sla"]);
    // A period without tickets is empty, not an error.
    let (status, r) = get(
        &app,
        SUPPORT,
        "/helpdesk/reports?from=2025-01-01&to=2025-01-03",
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(r["volume"].as_array().unwrap().len(), 3);
    // Merged tickets are left out.
    let (status, _) = post(
        &app,
        SUPPORT,
        &format!("/tickets/{}/merge", f.c),
        json!({ "target_id": f.a }),
    )
    .await;
    assert_eq!(
        status,
        StatusCode::CONFLICT,
        "different state: closed target"
    );
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn response_and_resolution_times_are_business_time_when_the_calendar_is_on(db: PgPool) {
    let app = setup(db.clone()).await;
    let f = fixture(&app, &db).await;
    // A UTC calendar keeps the arithmetic independent of summer time.
    let (status, _) = put(
        &app,
        ADMIN,
        "/helpdesk/calendar",
        json!({ "enabled": true, "time_zone": "UTC", "hours": office_hours() }),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let (_, r) = get(&app, SUPPORT, &format!("/helpdesk/reports?{}", f.range())).await;
    assert_eq!(r["basis"], "business");
    // A created 09:00 and closed after 4 h: 240. B created 10:00, closed 20:00: 7 business hours
    // (10:00-17:00) = 420.
    assert_eq!(num(&r["first_response"]["average_minutes"]), 60.0);
    assert_eq!(num(&r["resolution"]["average_minutes"]), 330.0);
    assert_eq!(num(&r["resolution"]["p90_minutes"]), 420.0);
    // Days are the company's local days.
    assert_eq!(r["volume"][1]["created"], 2);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn a_pause_is_not_part_of_the_resolution_time(db: PgPool) {
    let app = setup(db.clone()).await;
    let f = fixture(&app, &db).await;
    // A waited 100 minutes for the customer between 09:00 and 10:40 (closed after 240).
    sqlx::query(
        "INSERT INTO helpdesk.sla_pauses (id, company_id, ticket_id, started_at, ended_at)
         VALUES (gen_random_uuid(), $2::uuid, $1::uuid, $3::timestamptz, $4::timestamptz)",
    )
    .bind(&f.a)
    .bind(COMPANY)
    .bind(format!("{}T09:00:00Z", f.day(0)))
    .bind(format!("{}T10:40:00Z", f.day(0)))
    .execute(&db)
    .await
    .unwrap();
    sqlx::query("UPDATE helpdesk.tickets SET sla_paused_seconds = 6000 WHERE id = $1::uuid")
        .bind(&f.a)
        .execute(&db)
        .await
        .unwrap();
    let (_, r) = get(&app, SUPPORT, &format!("/helpdesk/reports?{}", f.range())).await;
    // A: 240 - 100 = 140, B: 600 -> average 370.
    assert_eq!(num(&r["resolution"]["average_minutes"]), 370.0);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn every_table_exports_as_csv(db: PgPool) {
    let app = setup(db.clone()).await;
    let f = fixture(&app, &db).await;
    let csv = |table: &str| {
        let app = app.clone();
        let table = table.to_string();
        let range = f.range();
        async move {
            let (status, headers, body) = raw(
                &app,
                Who::Staff(SUPPORT),
                Method::GET,
                &format!("/api/v1/helpdesk/reports/export?table={table}&{range}"),
                None,
            )
            .await;
            assert_eq!(status, StatusCode::OK, "{table}");
            assert!(headers["content-type"]
                .to_str()
                .unwrap()
                .starts_with("text/csv"));
            assert!(headers["content-disposition"]
                .to_str()
                .unwrap()
                .contains(&table));
            String::from_utf8(body).unwrap()
        }
    };
    let v = csv("volume").await;
    let lines: Vec<&str> = v.split("\r\n").filter(|l| !l.is_empty()).collect();
    assert_eq!(lines[0], "period,created,closed");
    assert_eq!(lines.len(), 8);
    assert_eq!(lines[2], format!("{},2,2", f.day(0)));
    assert_eq!(csv("times").await, "measure,basis,count,average_minutes,p90_minutes\r\nfirst_response,elapsed,2,60,90\r\nresolution,elapsed,2,420,600\r\n");
    assert_eq!(
        csv("backlog_status").await,
        "key,label,count\r\nopen,open,2\r\n"
    );
    assert!(csv("time_by_user").await.contains("Admin User,30,0,30"));
    assert!(csv("breaches_by_team")
        .await
        .starts_with("team,tickets,breached\r\nAlpha team,2,1\r\n"));
    // A tag that looks like a formula cannot run in a spreadsheet.
    assert!(csv("top_tags").await.contains("'=cmd,'=cmd,1"));
    for t in [
        "backlog_priority",
        "backlog_team",
        "backlog_owner",
        "sla",
        "top_categories",
    ] {
        assert!(csv(t).await.contains("\r\n"), "{t}");
    }
    let (status, _, _) = raw(
        &app,
        Who::Staff(SUPPORT),
        Method::GET,
        "/api/v1/helpdesk/reports/export?table=secrets",
        None,
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    let (status, _, _) = raw(
        &app,
        Who::Staff(TECH),
        Method::GET,
        "/api/v1/helpdesk/reports/export?table=volume",
        None,
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn reports_are_for_helpdesk_readers_and_for_one_company_only(db: PgPool) {
    let app = setup(db.clone()).await;
    second_company(&db).await;
    let f = fixture(&app, &db).await;
    for who in [ADMIN, SUPPORT, SALES, VIEWER] {
        let (status, _) = get(&app, who, &format!("/helpdesk/reports?{}", f.range())).await;
        assert_eq!(status, StatusCode::OK, "{who}");
    }
    assert_eq!(
        get(&app, TECH, "/helpdesk/reports").await.0,
        StatusCode::FORBIDDEN
    );
    // The other company sees nothing of ours.
    let (_, r) = get(&app, OTHER, &format!("/helpdesk/reports?{}", f.range())).await;
    assert_eq!(
        r["volume"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v["created"].as_i64().unwrap())
            .sum::<i64>(),
        0
    );
    assert_eq!(r["backlog_by_status"], json!([]));
    assert_eq!(r["time_by_user"], json!([]));
    assert_eq!(
        (
            r["csat"]["responses"].as_i64(),
            r["sla"]["tickets"].as_i64()
        ),
        (Some(0), Some(0))
    );
    assert_eq!(r["top_tags"], json!([]));
    // Our team id means nothing there.
    let (_, ours) = get(&app, ADMIN, "/helpdesk/teams").await;
    let team_id = ours[0]["id"].as_str().unwrap();
    let (_, r) = get(
        &app,
        OTHER,
        &format!("/helpdesk/reports?{}&team_id={team_id}", f.range()),
    )
    .await;
    assert_eq!(r["sla"]["tickets"], 0);
    // Bad parameters.
    for q in [
        "from=2026-11-07&to=2026-11-01",
        "from=2020-01-01&to=2026-11-01",
        "group=month",
        "category=nonsense",
    ] {
        let (status, _) = get(&app, SUPPORT, &format!("/helpdesk/reports?{q}")).await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY, "{q}");
    }
    let (status, _) = get(&app, SUPPORT, "/helpdesk/reports?from=garbage").await;
    assert!(status.is_client_error());
}
