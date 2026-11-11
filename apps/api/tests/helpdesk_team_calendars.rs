//! Helpdesk: a business calendar per team. A team without a calendar of its own uses the
//! company's; changes of a calendar or of a ticket's team recompute the due times they concern.

mod common;

use axum::http::{Method, StatusCode};
use chrono::{DateTime, Utc};
use common::{call, setup};
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

const ALPHA: &str = "00000000-0000-7000-8000-000000000401";
const ADMIN: &str = "admin@example.com";
const SUPPORT: &str = "support@example.com";
const OTHER: &str = "other@example.com";

/// Parses an RFC 3339 instant.
fn at(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s).unwrap().with_timezone(&Utc)
}

/// Reads a due time from a JSON value.
fn when(v: &Value) -> DateTime<Utc> {
    at(v.as_str().unwrap_or_else(|| panic!("not a date: {v}")))
}

/// Sends one request as `user` and returns the status and JSON body.
async fn api(
    app: &axum::Router,
    user: &str,
    method: Method,
    uri: &str,
    body: Option<Value>,
) -> (StatusCode, Value) {
    call(app, user, method, &format!("/api/v1{uri}"), body).await
}

/// POST helper.
async fn post(app: &axum::Router, user: &str, uri: &str, body: Value) -> (StatusCode, Value) {
    api(app, user, Method::POST, uri, Some(body)).await
}

/// PUT helper.
async fn put(app: &axum::Router, user: &str, uri: &str, body: Value) -> (StatusCode, Value) {
    api(app, user, Method::PUT, uri, Some(body)).await
}

/// GET helper.
async fn get(app: &axum::Router, user: &str, uri: &str) -> (StatusCode, Value) {
    api(app, user, Method::GET, uri, None).await
}

/// DELETE helper.
async fn delete(app: &axum::Router, user: &str, uri: &str) -> (StatusCode, Value) {
    api(app, user, Method::DELETE, uri, None).await
}

/// A one-day opening hours payload for the weekday tests.
fn hours(start: &str, end: &str) -> Value {
    json!([1, 2, 3, 4, 5].map(|d| json!({ "weekday": d, "start_time": start, "end_time": end })))
}

/// Saves a calendar for the company (`team` = None) or a team.
/// Saves a company or team calendar with one opening interval.
async fn save_calendar(
    app: &axum::Router,
    team: Option<&str>,
    enabled: bool,
    start: &str,
    end: &str,
) -> (StatusCode, Value) {
    let q = team.map(|t| format!("?team_id={t}")).unwrap_or_default();
    put(
        app,
        ADMIN,
        &format!("/helpdesk/calendar{q}"),
        json!({ "enabled": enabled, "time_zone": "Europe/Brussels", "hours": hours(start, end) }),
    )
    .await
}

/// Creates a team and returns its id.
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

/// A normal-priority ticket created on Tuesday 10 November 2026, 16:00 local (15:00 UTC), in
/// the given team. Its first-response window is 8 business hours.
/// Creates a ticket in a team and returns its id.
async fn ticket_in(app: &axum::Router, db: &PgPool, team: Option<&str>) -> String {
    let (status, t) = post(
        app,
        SUPPORT,
        "/tickets",
        json!({ "subject": "Printer", "customer_id": ALPHA, "contact_email": "anna@alpha.example", "body": "Help" }),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{t}");
    let id = t["ticket"]["id"].as_str().unwrap().to_string();
    sqlx::query("UPDATE helpdesk.tickets SET created_at = '2026-11-10T15:00:00Z', team_id = $2::uuid WHERE id = $1::uuid")
        .bind(&id)
        .bind(team)
        .execute(db)
        .await
        .unwrap();
    // Stale due times: the tests save a calendar to have them recomputed.
    sqlx::query("UPDATE helpdesk.tickets SET first_response_due_at = '2030-01-01T00:00:00Z' WHERE id = $1::uuid")
        .bind(&id)
        .execute(db)
        .await
        .unwrap();
    id
}

/// The first-response due time stored on a ticket.
async fn first_due(db: &PgPool, id: &str) -> DateTime<Utc> {
    sqlx::query_scalar("SELECT first_response_due_at FROM helpdesk.tickets WHERE id = $1::uuid")
        .bind(id)
        .fetch_one(db)
        .await
        .unwrap()
}

/// Sets a sentinel due time so a later recompute is visible.
async fn mark(db: &PgPool, ids: &[&String]) {
    for id in ids {
        sqlx::query("UPDATE helpdesk.tickets SET first_response_due_at = '2030-01-01T00:00:00Z' WHERE id = $1::uuid")
            .bind(id.as_str())
            .execute(db)
            .await
            .unwrap();
    }
}

/// Assigns a ticket to a team (or none).
async fn assign(app: &axum::Router, id: &str, team: Option<&str>) {
    let (status, r) = put(
        app,
        SUPPORT,
        &format!("/tickets/{id}/assignment"),
        json!({ "team_id": team, "owner_user_id": null }),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{r}");
}

/// The marker due time written by `mark`.
fn sentinel() -> DateTime<Utc> {
    at("2030-01-01T00:00:00Z")
}

const COMPANY_DUE: &str = "2026-11-11T15:00:00Z"; // 09:00-17:00: 1 h Tuesday + 7 h Wednesday
const TEAM_DUE: &str = "2026-11-13T10:00:00Z"; // 09:00-12:00: 3 h Wednesday, Thursday, 2 h Friday

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn a_team_calendar_overrides_the_company_one_and_is_inherited_otherwise(db: PgPool) {
    let app = setup(db.clone()).await;
    let t = team(&app, "Short days").await;
    let (status, _) = save_calendar(&app, None, true, "09:00", "17:00").await;
    assert_eq!(status, StatusCode::OK);

    // No calendar of its own: the team inherits the company's.
    let (_, c) = get(&app, ADMIN, &format!("/helpdesk/calendar?team_id={t}")).await;
    assert_eq!(c["inherited"], true);
    assert_eq!(c["team_id"], t.as_str());
    assert_eq!(c["hours"][0]["end_time"], "17:00");
    let id = ticket_in(&app, &db, Some(&t)).await;
    save_calendar(&app, None, true, "09:00", "17:00").await;
    assert_eq!(first_due(&db, &id).await, at(COMPANY_DUE));

    // Its own calendar takes over; the company calendar is untouched.
    let (status, c) = save_calendar(&app, Some(&t), true, "09:00", "12:00").await;
    assert_eq!(status, StatusCode::OK, "{c}");
    assert_eq!(c["inherited"], false);
    assert_eq!(first_due(&db, &id).await, at(TEAM_DUE));
    let (_, company) = get(&app, ADMIN, "/helpdesk/calendar").await;
    assert_eq!(company["hours"][0]["end_time"], "17:00");
    assert_eq!(company["team_id"], Value::Null);
    let (_, detail) = get(&app, SUPPORT, &format!("/tickets/{id}")).await;
    assert_eq!(
        when(&detail["ticket"]["sla"]["first_response"]["due_at"]),
        at(TEAM_DUE)
    );

    // A team calendar that is off means plain elapsed time for that team only.
    save_calendar(&app, Some(&t), false, "09:00", "12:00").await;
    assert_eq!(
        first_due(&db, &id).await,
        at("2026-11-10T15:00:00Z") + chrono::Duration::hours(8)
    );

    // Removing it hands the team back to the company calendar.
    let (status, c) = delete(&app, ADMIN, &format!("/helpdesk/calendar?team_id={t}")).await;
    assert_eq!(status, StatusCode::OK, "{c}");
    assert_eq!(c["inherited"], true);
    assert_eq!(first_due(&db, &id).await, at(COMPANY_DUE));
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn team_holidays_add_to_the_company_holidays(db: PgPool) {
    let app = setup(db.clone()).await;
    let t = team(&app, "Desk").await;
    save_calendar(&app, None, true, "09:00", "17:00").await;
    let (status, _) = post(
        &app,
        ADMIN,
        "/helpdesk/calendar/holidays",
        json!({ "day": "2026-11-11", "name": "Armistice" }),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);

    // A team without a calendar cannot hold holidays of its own.
    let (status, _) = post(
        &app,
        ADMIN,
        &format!("/helpdesk/calendar/holidays?team_id={t}"),
        json!({ "day": "2026-11-12", "name": "Team day" }),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);

    let id = ticket_in(&app, &db, Some(&t)).await;
    save_calendar(&app, Some(&t), true, "09:00", "17:00").await;
    // Its calendar has no holidays yet: only the company's count (Wednesday is closed).
    let (_, c) = get(&app, ADMIN, &format!("/helpdesk/calendar?team_id={t}")).await;
    assert_eq!(c["holidays_inherited"], true);
    assert_eq!(c["holidays"].as_array().unwrap().len(), 0);
    assert_eq!(first_due(&db, &id).await, at("2026-11-12T15:00:00Z"));

    // A holiday of its own adds to the company's for this team.
    let (status, h) = post(
        &app,
        ADMIN,
        &format!("/helpdesk/calendar/holidays?team_id={t}"),
        json!({ "day": "2026-11-12", "name": "Team day" }),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{h}");
    let (_, c) = get(&app, ADMIN, &format!("/helpdesk/calendar?team_id={t}")).await;
    assert_eq!(c["holidays_inherited"], true);
    assert_eq!(c["holidays"].as_array().unwrap().len(), 1);
    assert_eq!(c["holidays"][0]["name"], "Team day");
    assert_eq!(
        first_due(&db, &id).await,
        at("2026-11-13T15:00:00Z"),
        "Wednesday and Thursday are both closed"
    );

    // The company calendar still shows only its own holiday.
    let (_, company) = get(&app, ADMIN, "/helpdesk/calendar").await;
    assert_eq!(company["holidays"].as_array().unwrap().len(), 1);
    assert_eq!(company["holidays"][0]["name"], "Armistice");

    // Removing the team's holiday leaves the company's.
    let hid = h["id"].as_str().unwrap();
    let (status, _) = delete(&app, ADMIN, &format!("/helpdesk/calendar/holidays/{hid}")).await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    assert_eq!(first_due(&db, &id).await, at("2026-11-12T15:00:00Z"));
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn changing_the_team_of_a_ticket_recomputes_its_due_times(db: PgPool) {
    let app = setup(db.clone()).await;
    let short = team(&app, "Short days").await;
    save_calendar(&app, None, true, "09:00", "17:00").await;
    save_calendar(&app, Some(&short), true, "09:00", "12:00").await;
    let id = ticket_in(&app, &db, None).await;

    assign(&app, &id, Some(&short)).await;
    assert_eq!(first_due(&db, &id).await, at(TEAM_DUE));
    assign(&app, &id, None).await;
    assert_eq!(first_due(&db, &id).await, at(COMPANY_DUE));

    // The risk instants move with it, so the at-risk filter agrees.
    assign(&app, &id, Some(&short)).await;
    let risk: DateTime<Utc> = sqlx::query_scalar(
        "SELECT first_response_risk_at FROM helpdesk.tickets WHERE id = $1::uuid",
    )
    .bind(&id)
    .fetch_one(&db)
    .await
    .unwrap();
    assert!(risk < at(TEAM_DUE) && risk > at("2026-11-10T15:00:00Z"));
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn an_automation_rule_that_sets_the_team_recomputes_the_due_times(db: PgPool) {
    let app = setup(db.clone()).await;
    let short = team(&app, "Short days").await;
    save_calendar(&app, None, true, "09:00", "17:00").await;
    save_calendar(&app, Some(&short), true, "09:00", "12:00").await;
    let (status, r) = post(
        &app,
        ADMIN,
        "/helpdesk/automation/rules",
        json!({ "name": "To short days", "trigger": "updated",
                "actions": [{ "kind": "set_team", "team_id": short }] }),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{r}");
    let id = ticket_in(&app, &db, None).await;
    let (_, d) = get(&app, SUPPORT, &format!("/tickets/{id}")).await;
    let t = &d["ticket"];
    let (status, r) = put(
        &app,
        SUPPORT,
        &format!("/tickets/{id}"),
        json!({
            "subject": t["subject"], "status": t["status"], "priority": t["priority"],
            "category": t["category"], "language": t["language"], "owner_user_id": t["owner_user_id"],
            "customer_id": t["customer_id"], "contact_id": t["contact_id"],
            "contact_name": t["contact_name"], "contact_email": t["contact_email"],
        }),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{r}");
    assert_eq!(r["ticket"]["team_id"], short.as_str());
    assert_eq!(first_due(&db, &id).await, at(TEAM_DUE));
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn a_calendar_edit_recomputes_only_the_tickets_it_applies_to(db: PgPool) {
    let app = setup(db.clone()).await;
    let own = team(&app, "Own calendar").await;
    let inheriting = team(&app, "Inheriting").await;
    save_calendar(&app, None, true, "09:00", "17:00").await;
    save_calendar(&app, Some(&own), true, "09:00", "12:00").await;
    let no_team = ticket_in(&app, &db, None).await;
    let in_own = ticket_in(&app, &db, Some(&own)).await;
    let in_inheriting = ticket_in(&app, &db, Some(&inheriting)).await;
    let all = [&no_team, &in_own, &in_inheriting];

    // The company hours: tickets without a team and of teams without a calendar; not the team
    // that has its own.
    mark(&db, &all).await;
    save_calendar(&app, None, true, "09:00", "17:00").await;
    assert_eq!(first_due(&db, &no_team).await, at(COMPANY_DUE));
    assert_eq!(first_due(&db, &in_inheriting).await, at(COMPANY_DUE));
    assert_eq!(first_due(&db, &in_own).await, sentinel());

    // The team's calendar: only its own tickets.
    mark(&db, &all).await;
    save_calendar(&app, Some(&own), true, "09:00", "12:00").await;
    assert_eq!(first_due(&db, &in_own).await, at(TEAM_DUE));
    assert_eq!(first_due(&db, &no_team).await, sentinel());
    assert_eq!(first_due(&db, &in_inheriting).await, sentinel());

    // A company holiday reaches tickets without a team, of teams without a calendar, and of
    // teams whose calendar has no holidays of its own.
    mark(&db, &all).await;
    let (status, h) = post(
        &app,
        ADMIN,
        "/helpdesk/calendar/holidays",
        json!({ "day": "2026-11-11", "name": "Armistice" }),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{h}");
    assert_ne!(first_due(&db, &no_team).await, sentinel());
    assert_ne!(first_due(&db, &in_inheriting).await, sentinel());
    assert_ne!(first_due(&db, &in_own).await, sentinel());

    // A team with holidays of its own still observes the company's.
    let (status, _) = post(
        &app,
        ADMIN,
        &format!("/helpdesk/calendar/holidays?team_id={own}"),
        json!({ "day": "2026-12-25", "name": "Christmas" }),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    mark(&db, &all).await;
    let (status, _) = post(
        &app,
        ADMIN,
        "/helpdesk/calendar/holidays",
        json!({ "day": "2026-12-24", "name": "Eve" }),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_ne!(first_due(&db, &no_team).await, sentinel());
    assert_ne!(first_due(&db, &in_own).await, sentinel());

    // A holiday of the team reaches its tickets only.
    mark(&db, &all).await;
    post(
        &app,
        ADMIN,
        &format!("/helpdesk/calendar/holidays?team_id={own}"),
        json!({ "day": "2026-12-26", "name": "Boxing day" }),
    )
    .await;
    assert_ne!(first_due(&db, &in_own).await, sentinel());
    assert_eq!(first_due(&db, &no_team).await, sentinel());
    assert_eq!(first_due(&db, &in_inheriting).await, sentinel());
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn calendars_are_admin_only_and_isolated_per_company(db: PgPool) {
    let app = setup(db.clone()).await;
    let ours = team(&app, "Ours").await;
    save_calendar(&app, Some(&ours), true, "09:00", "12:00").await;
    let (_, h) = post(
        &app,
        ADMIN,
        &format!("/helpdesk/calendar/holidays?team_id={ours}"),
        json!({ "day": "2026-11-11", "name": "Armistice" }),
    )
    .await;
    let hid = h["id"].as_str().unwrap().to_string();
    let id = ticket_in(&app, &db, Some(&ours)).await;
    save_calendar(&app, Some(&ours), true, "09:00", "12:00").await;
    let due = first_due(&db, &id).await;

    // Support staff are not helpdesk administrators.
    let (status, _) = get(&app, SUPPORT, &format!("/helpdesk/calendar?team_id={ours}")).await;
    assert_eq!(status, StatusCode::FORBIDDEN);

    // Another company's administrator.
    let company = Uuid::now_v7();
    sqlx::query("INSERT INTO core.companies (id, name, country_code, currency) VALUES ($1, 'Other BV', 'BE', 'EUR')")
        .bind(company)
        .execute(&db)
        .await
        .unwrap();
    let user = Uuid::now_v7();
    sqlx::query("INSERT INTO core.users (id, email, display_name) VALUES ($1, $2, 'Other Admin')")
        .bind(user)
        .bind(OTHER)
        .execute(&db)
        .await
        .unwrap();
    sqlx::query("INSERT INTO core.user_companies (user_id, company_id, roles, is_default) VALUES ($1, $2, '{admin}', true)")
        .bind(user)
        .bind(company)
        .execute(&db)
        .await
        .unwrap();

    let q = format!("?team_id={ours}");
    let (status, _) = get(&app, OTHER, &format!("/helpdesk/calendar{q}")).await;
    assert_eq!(status, StatusCode::NOT_FOUND, "our team is unknown to them");
    let (status, _) = put(
        &app,
        OTHER,
        &format!("/helpdesk/calendar{q}"),
        json!({ "enabled": true, "time_zone": "Europe/Brussels", "hours": hours("09:00", "10:00") }),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    let (status, _) = post(
        &app,
        OTHER,
        &format!("/helpdesk/calendar/holidays{q}"),
        json!({ "day": "2026-11-12", "name": "x" }),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    let (status, _) = delete(&app, OTHER, &format!("/helpdesk/calendar{q}")).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    let (status, _) = delete(&app, OTHER, &format!("/helpdesk/calendar/holidays/{hid}")).await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    // Their own company calendar is separate and leaves our tickets alone.
    let (status, _) = put(
        &app,
        OTHER,
        "/helpdesk/calendar",
        json!({ "enabled": true, "time_zone": "Europe/Brussels", "hours": hours("09:00", "10:00") }),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let (_, theirs) = get(&app, OTHER, "/helpdesk/calendar").await;
    assert_eq!(theirs["hours"][0]["end_time"], "10:00");
    let (_, ours_now) = get(&app, ADMIN, &format!("/helpdesk/calendar{q}")).await;
    assert_eq!(ours_now["hours"][0]["end_time"], "12:00");
    assert_eq!(ours_now["holidays"].as_array().unwrap().len(), 1);
    assert_eq!(first_due(&db, &id).await, due);
    let ours_company: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM helpdesk.business_calendars WHERE team_id IS NULL AND company_id <> $1",
    )
    .bind(company)
    .fetch_one(&db)
    .await
    .unwrap();
    assert_eq!(ours_company, 0);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn reports_measure_each_ticket_with_the_calendar_of_its_team(db: PgPool) {
    let app = setup(db.clone()).await;
    let short = team(&app, "Short days").await;
    save_calendar(&app, None, true, "09:00", "17:00").await;
    save_calendar(&app, Some(&short), true, "10:00", "11:00").await;
    let company_ticket = ticket_in(&app, &db, None).await;
    let team_ticket = ticket_in(&app, &db, Some(&short)).await;
    // Both answered Wednesday 11 November 09:00 local, 08:00 UTC; created Tuesday 16:00 local.
    for id in [&company_ticket, &team_ticket] {
        sqlx::query("UPDATE helpdesk.tickets SET first_response_at = '2026-11-11T08:00:00Z' WHERE id = $1::uuid")
            .bind(id)
            .execute(&db)
            .await
            .unwrap();
    }
    let (status, r) = get(
        &app,
        SUPPORT,
        "/helpdesk/reports?from=2026-11-09&to=2026-11-15",
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{r}");
    assert_eq!(r["first_response"]["count"], 2);
    // Company calendar: Tuesday 16:00-17:00 = 60 min. Team calendar (10:00-11:00): none before
    // Wednesday 10:00, so 0 min at 09:00. Average 30.
    assert_eq!(r["first_response"]["average_minutes"].as_f64(), Some(30.0));
    assert_eq!(r["basis"], "business");

    // One team with a calendar, one without: a mix.
    save_calendar(&app, Some(&short), false, "10:00", "11:00").await;
    let (_, r) = get(
        &app,
        SUPPORT,
        "/helpdesk/reports?from=2026-11-09&to=2026-11-15",
    )
    .await;
    assert_eq!(r["basis"], "mixed");
    // Filtered to the team whose calendar is off: elapsed, 17 hours.
    let (_, r) = get(
        &app,
        SUPPORT,
        &format!("/helpdesk/reports?from=2026-11-09&to=2026-11-15&team_id={short}"),
    )
    .await;
    assert_eq!(r["basis"], "elapsed");
    assert_eq!(
        r["first_response"]["average_minutes"].as_f64(),
        Some(1020.0)
    );
}
