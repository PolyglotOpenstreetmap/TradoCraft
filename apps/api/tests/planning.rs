//! Planning board: scheduling and assigning, conflicts, suggestions, mail, isolation, permissions.

mod common;

use axum::http::{Method, StatusCode};
use common::{call, setup};
use serde_json::{json, Value};
use sqlx::PgPool;

const ALPHA: &str = "00000000-0000-7000-8000-000000000401";
const TOM: &str = "00000000-0000-7000-8000-000000000107"; // technician@example.com
const DINA: &str = "00000000-0000-7000-8000-000000000108"; // dispatcher@example.com
const DISPATCHER: &str = "dispatcher@example.com";
const TECH: &str = "technician@example.com";
const ADMIN: &str = "admin@example.com";
const VIEWER: &str = "viewer@example.com";
const SALES: &str = "sales@example.com";

async fn get(app: &axum::Router, user: &str, uri: &str) -> (StatusCode, Value) {
    call(app, user, Method::GET, &format!("/api/v1/{uri}"), None).await
}
async fn put(app: &axum::Router, user: &str, uri: &str, body: Value) -> (StatusCode, Value) {
    call(
        app,
        user,
        Method::PUT,
        &format!("/api/v1/{uri}"),
        Some(body),
    )
    .await
}
async fn post(app: &axum::Router, user: &str, uri: &str, body: Value) -> (StatusCode, Value) {
    call(
        app,
        user,
        Method::POST,
        &format!("/api/v1/{uri}"),
        Some(body),
    )
    .await
}

/// An open job without schedule or technician.
async fn new_job(app: &axum::Router, title: &str) -> String {
    let (s, j) = post(
        app,
        DISPATCHER,
        "field/jobs",
        json!({ "title": title, "kind": "maintenance", "customer_id": ALPHA }),
    )
    .await;
    assert_eq!(s, StatusCode::CREATED, "{j}");
    j["job"]["id"].as_str().unwrap().to_string()
}

async fn second_technician(db: &PgPool, language: Option<&str>) -> String {
    let id = uuid::Uuid::now_v7();
    sqlx::query("INSERT INTO core.users (id, email, display_name, language) VALUES ($1, 'tina@example.com', 'Tina Technician', $2)")
        .bind(id)
        .bind(language)
        .execute(db)
        .await
        .unwrap();
    sqlx::query("INSERT INTO core.user_companies (user_id, company_id, roles, is_default) VALUES ($1, '00000000-0000-7000-8000-000000000001', '{technician}', true)")
        .bind(id)
        .execute(db)
        .await
        .unwrap();
    id.to_string()
}

/// Monday to Friday 08:00-12:00 and 13:00-17:00 for the company.
async fn office_hours(app: &axum::Router) {
    let slots: Vec<Value> = (1..=5)
        .flat_map(|w| {
            [
                json!({ "weekday": w, "start_time": "08:00", "end_time": "12:00" }),
                json!({ "weekday": w, "start_time": "13:00", "end_time": "17:00" }),
            ]
        })
        .collect();
    let (s, b) = put(
        app,
        ADMIN,
        "planning/working-hours",
        json!({ "slots": slots }),
    )
    .await;
    assert_eq!(s, StatusCode::OK, "{b}");
}

/// Local times in November are UTC+1: Monday 9 November 09:00 local is 08:00Z.
fn slot(from: &str, to: &str, techs: &[&str]) -> Value {
    json!({ "scheduled_start": from, "scheduled_end": to, "technician_ids": techs })
}

async fn schedule(app: &axum::Router, job: &str, query: &str, body: Value) -> (StatusCode, Value) {
    put(
        app,
        DISPATCHER,
        &format!("planning/jobs/{job}/schedule{query}"),
        body,
    )
    .await
}

fn kinds(r: &Value) -> Vec<String> {
    r["conflicts"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c["kind"].as_str().unwrap().to_string())
        .collect()
}

async fn mails(db: &PgPool, to: &str) -> Vec<(String, String)> {
    sqlx::query_as(
        "SELECT subject, body FROM core.mail_outbox WHERE to_address = $1 ORDER BY created_at, id",
    )
    .bind(to)
    .fetch_all(db)
    .await
    .unwrap()
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn scheduling_assigns_technicians_and_records_an_audit_event(db: PgPool) {
    let app = setup(db.clone()).await;
    office_hours(&app).await;
    let job = new_job(&app, "Service pump").await;

    let (s, r) = schedule(
        &app,
        &job,
        "",
        slot("2026-11-09T08:00:00Z", "2026-11-09T10:00:00Z", &[TOM]),
    )
    .await;
    assert_eq!(s, StatusCode::OK, "{r}");
    assert_eq!(r["applied"], true);
    assert!(r["conflicts"].as_array().unwrap().is_empty());
    assert_eq!(r["job"]["technician_ids"], json!([TOM]));

    // The field module sees the same facts, and Tom sees the job in his own list.
    let (_, d) = get(&app, DISPATCHER, &format!("field/jobs/{job}")).await;
    assert_eq!(d["job"]["technicians"][0]["user_id"], TOM);
    assert_eq!(d["job"]["scheduled_start"], "2026-11-09T08:00:00Z");
    let (_, mine) = get(&app, TECH, "field/my-jobs").await;
    assert_eq!(mine.as_array().unwrap().len(), 1);

    let audits: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM audit.events WHERE entity = 'job' AND action = 'scheduled'",
    )
    .fetch_one(&db)
    .await
    .unwrap();
    assert_eq!(audits, 1);

    // It is on the board for that week, and no longer in the backlog.
    let (s, b) = get(
        &app,
        DISPATCHER,
        "planning/board?from=2026-11-09&to=2026-11-15",
    )
    .await;
    assert_eq!(s, StatusCode::OK, "{b}");
    assert_eq!(b["jobs"].as_array().unwrap().len(), 1);
    assert!(b["backlog"].as_array().unwrap().is_empty());
    let other_week = get(
        &app,
        DISPATCHER,
        "planning/board?from=2026-11-16&to=2026-11-22",
    )
    .await
    .1;
    assert!(other_week["jobs"].as_array().unwrap().is_empty());

    // Rescheduling with another technician replaces the assignment.
    let tina = second_technician(&db, None).await;
    let (s, _) = schedule(
        &app,
        &job,
        "",
        slot("2026-11-09T08:00:00Z", "2026-11-09T10:00:00Z", &[&tina]),
    )
    .await;
    assert_eq!(s, StatusCode::OK);
    let (_, mine) = get(&app, TECH, "field/my-jobs").await;
    assert!(mine.as_array().unwrap().is_empty());

    // Validation.
    let (s, _) = schedule(
        &app,
        &job,
        "",
        slot("2026-11-09T10:00:00Z", "2026-11-09T08:00:00Z", &[TOM]),
    )
    .await;
    assert_eq!(s, StatusCode::UNPROCESSABLE_ENTITY);
    let (s, _) = schedule(
        &app,
        &job,
        "",
        slot(
            "2026-11-09T08:00:00Z",
            "2026-11-09T10:00:00Z",
            &[DINA, "00000000-0000-7000-8000-0000000000ff"],
        ),
    )
    .await;
    assert_eq!(s, StatusCode::UNPROCESSABLE_ENTITY);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn a_dry_run_reports_conflicts_and_changes_nothing(db: PgPool) {
    let app = setup(db.clone()).await;
    office_hours(&app).await;
    let first = new_job(&app, "First").await;
    schedule(
        &app,
        &first,
        "",
        slot("2026-11-09T08:00:00Z", "2026-11-09T10:00:00Z", &[TOM]),
    )
    .await;
    let second = new_job(&app, "Second").await;
    let before = mails(&db, "technician@example.com").await.len();

    let (s, r) = schedule(
        &app,
        &second,
        "?dry_run=true",
        slot("2026-11-09T09:00:00Z", "2026-11-09T11:00:00Z", &[TOM]),
    )
    .await;
    assert_eq!(s, StatusCode::OK, "{r}");
    assert_eq!(r["applied"], false);
    assert_eq!(kinds(&r), vec!["double_booked"]);
    assert_eq!(r["conflicts"][0]["severity"], "error");
    assert_eq!(r["conflicts"][0]["technician_id"], TOM);
    assert_eq!(r["conflicts"][0]["other_job_id"], first);
    assert_eq!(
        r["conflicts"][0]["message_key"],
        "planning.conflict.double_booked"
    );

    let (_, d) = get(&app, DISPATCHER, &format!("field/jobs/{second}")).await;
    assert!(d["job"]["scheduled_start"].is_null());
    assert!(d["job"]["technicians"].as_array().unwrap().is_empty());
    assert_eq!(mails(&db, "technician@example.com").await.len(), before);
    let audits: i64 =
        sqlx::query_scalar("SELECT count(*) FROM audit.events WHERE action = 'scheduled'")
            .fetch_one(&db)
            .await
            .unwrap();
    assert_eq!(audits, 1);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn double_booking_includes_the_travel_buffer_and_needs_force(db: PgPool) {
    let app = setup(db.clone()).await;
    office_hours(&app).await;
    let first = new_job(&app, "First").await;
    schedule(
        &app,
        &first,
        "",
        slot("2026-11-09T08:00:00Z", "2026-11-09T09:00:00Z", &[TOM]),
    )
    .await;
    let second = new_job(&app, "Second").await;

    // The default buffer is 15 minutes: starting 10 minutes after the first ends is too tight.
    let tight = slot("2026-11-09T09:10:00Z", "2026-11-09T10:00:00Z", &[TOM]);
    let (s, e) = schedule(&app, &second, "", tight.clone()).await;
    assert_eq!(s, StatusCode::CONFLICT, "{e}");
    let (s, r) = schedule(&app, &second, "?force=true", tight).await;
    assert_eq!(s, StatusCode::OK);
    assert_eq!(kinds(&r), vec!["double_booked"]);
    assert_eq!(r["applied"], true);

    // A bigger buffer from the settings makes a previously fine slot conflict.
    let third = new_job(&app, "Third").await;
    let fine = slot("2026-11-09T10:15:00Z", "2026-11-09T11:00:00Z", &[TOM]);
    let (_, r) = schedule(&app, &third, "?dry_run=true", fine.clone()).await;
    assert!(r["conflicts"].as_array().unwrap().is_empty());
    let (s, st) = put(&app, ADMIN, "planning/settings", json!({ "travel_buffer_minutes": 60, "default_job_minutes": 120, "timezone": "Europe/Brussels" })).await;
    assert_eq!(s, StatusCode::OK, "{st}");
    let (_, r) = schedule(&app, &third, "?dry_run=true", fine).await;
    assert_eq!(kinds(&r), vec!["double_booked"]);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn working_hours_skills_and_crossing_days_are_warnings(db: PgPool) {
    let app = setup(db.clone()).await;
    office_hours(&app).await;
    let job = new_job(&app, "Install").await;

    // 16:00-18:00 local (15:00Z-17:00Z) runs past the end of the working day.
    let (s, r) = schedule(
        &app,
        &job,
        "",
        slot("2026-11-09T15:00:00Z", "2026-11-09T17:00:00Z", &[TOM]),
    )
    .await;
    assert_eq!(s, StatusCode::OK, "{r}");
    assert_eq!(kinds(&r), vec!["outside_hours"]);
    assert_eq!(r["conflicts"][0]["severity"], "warning");
    // The lunch break counts as outside hours too.
    let (_, r) = schedule(
        &app,
        &job,
        "?dry_run=true",
        slot("2026-11-09T10:30:00Z", "2026-11-09T12:30:00Z", &[TOM]),
    )
    .await;
    assert_eq!(kinds(&r), vec!["outside_hours"]);
    // Weekend.
    let (_, r) = schedule(
        &app,
        &job,
        "?dry_run=true",
        slot("2026-11-14T08:00:00Z", "2026-11-14T09:00:00Z", &[TOM]),
    )
    .await;
    assert_eq!(kinds(&r), vec!["outside_hours"]);

    // Over midnight: flagged for the job, not for a technician.
    let (_, r) = schedule(
        &app,
        &job,
        "?dry_run=true",
        slot("2026-11-09T15:00:00Z", "2026-11-10T08:00:00Z", &[TOM]),
    )
    .await;
    let crossing = r["conflicts"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["kind"] == "crosses_days")
        .unwrap();
    assert!(crossing["technician_id"].is_null());
    assert_eq!(crossing["severity"], "warning");

    // A required skill Tom lacks.
    let (s, sk) = post(
        &app,
        ADMIN,
        "planning/skills",
        json!({ "code": "HVAC", "name": "HVAC" }),
    )
    .await;
    assert_eq!(s, StatusCode::CREATED, "{sk}");
    let skill = sk["id"].as_str().unwrap();
    let (s, _) = put(
        &app,
        DISPATCHER,
        &format!("planning/jobs/{job}/skills"),
        json!({ "skill_ids": [skill] }),
    )
    .await;
    assert_eq!(s, StatusCode::OK);
    let ok = slot("2026-11-09T08:00:00Z", "2026-11-09T10:00:00Z", &[TOM]);
    let (_, r) = schedule(&app, &job, "?dry_run=true", ok.clone()).await;
    assert_eq!(kinds(&r), vec!["missing_skill"]);
    assert_eq!(r["conflicts"][0]["skill_id"], skill);
    assert_eq!(r["conflicts"][0]["severity"], "warning");
    // Once Tom has the skill the warning is gone.
    let (s, _) = put(
        &app,
        ADMIN,
        &format!("planning/users/{TOM}/skills"),
        json!({ "skills": [{ "skill_id": skill, "level": 2 }] }),
    )
    .await;
    assert_eq!(s, StatusCode::OK);
    let (_, r) = schedule(&app, &job, "?dry_run=true", ok).await;
    assert!(r["conflicts"].as_array().unwrap().is_empty());
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn an_absence_is_an_error_unless_forced(db: PgPool) {
    let app = setup(db.clone()).await;
    office_hours(&app).await;
    let (s, a) = post(&app, DISPATCHER, "planning/absences", json!({
        "user_id": TOM, "from_date": "2026-11-10", "to_date": "2026-11-12", "kind": "vacation", "note": "Ardennes"
    })).await;
    assert_eq!(s, StatusCode::CREATED, "{a}");
    let job = new_job(&app, "Service").await;
    let on_leave = slot("2026-11-11T08:00:00Z", "2026-11-11T10:00:00Z", &[TOM]);

    let (_, r) = schedule(&app, &job, "?dry_run=true", on_leave.clone()).await;
    assert_eq!(kinds(&r), vec!["on_absence"]);
    assert_eq!(r["conflicts"][0]["severity"], "error");
    let (s, _) = schedule(&app, &job, "", on_leave.clone()).await;
    assert_eq!(s, StatusCode::CONFLICT);
    let (_, d) = get(&app, DISPATCHER, &format!("field/jobs/{job}")).await;
    assert!(d["job"]["scheduled_start"].is_null());

    let (s, r) = schedule(&app, &job, "?force=true", on_leave).await;
    assert_eq!(s, StatusCode::OK);
    assert_eq!(r["applied"], true);
    assert_eq!(kinds(&r), vec!["on_absence"]);
    let forced: bool = sqlx::query_scalar(
        "SELECT (data->>'forced')::boolean FROM audit.events WHERE action = 'scheduled'",
    )
    .fetch_one(&db)
    .await
    .unwrap();
    assert!(forced);

    // The board shows the absence of the week.
    let (_, b) = get(
        &app,
        DISPATCHER,
        "planning/board?from=2026-11-09&to=2026-11-15",
    )
    .await;
    let tom = b["technicians"]
        .as_array()
        .unwrap()
        .iter()
        .find(|t| t["user_id"] == TOM)
        .unwrap();
    assert_eq!(tom["absences"][0]["kind"], "vacation");
    assert_eq!(tom["absences"][0]["to_date"], "2026-11-12");
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn completed_and_cancelled_jobs_cannot_be_scheduled(db: PgPool) {
    let app = setup(db.clone()).await;
    let done = new_job(&app, "Done").await;
    let cancelled = new_job(&app, "Cancelled").await;
    sqlx::query(
        "UPDATE field.jobs SET status = 'completed', completed_at = now() WHERE id = $1::uuid",
    )
    .bind(&done)
    .execute(&db)
    .await
    .unwrap();
    sqlx::query("UPDATE field.jobs SET status = 'cancelled' WHERE id = $1::uuid")
        .bind(&cancelled)
        .execute(&db)
        .await
        .unwrap();
    for job in [&done, &cancelled] {
        for q in ["", "?dry_run=true"] {
            let (s, e) = schedule(
                &app,
                job,
                q,
                slot("2026-11-09T08:00:00Z", "2026-11-09T10:00:00Z", &[TOM]),
            )
            .await;
            assert_eq!(s, StatusCode::CONFLICT, "{e}");
        }
    }
    // Neither is in the backlog.
    let (_, b) = get(
        &app,
        DISPATCHER,
        "planning/board?from=2026-11-09&to=2026-11-15",
    )
    .await;
    assert!(b["backlog"].as_array().unwrap().is_empty());
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn suggestions_rank_skilled_free_technicians_first(db: PgPool) {
    let app = setup(db.clone()).await;
    office_hours(&app).await;
    let tina = second_technician(&db, None).await;
    let (_, sk) = post(
        &app,
        ADMIN,
        "planning/skills",
        json!({ "code": "PLC", "name": "PLC programming" }),
    )
    .await;
    let skill = sk["id"].as_str().unwrap().to_string();
    put(
        &app,
        ADMIN,
        &format!("planning/users/{tina}/skills"),
        json!({ "skills": [{ "skill_id": skill, "level": 3 }] }),
    )
    .await;

    let job = new_job(&app, "Commission PLC").await;
    put(
        &app,
        DISPATCHER,
        &format!("planning/jobs/{job}/skills"),
        json!({ "skill_ids": [skill] }),
    )
    .await;
    // Tina is busy on Monday morning, Tom is free: Tina still ranks first thanks to the skill.
    let busy = new_job(&app, "Busy").await;
    schedule(
        &app,
        &busy,
        "",
        slot("2026-11-09T07:00:00Z", "2026-11-09T11:00:00Z", &[&tina]),
    )
    .await;

    let (s, r) = get(
        &app,
        DISPATCHER,
        &format!("planning/suggestions/{job}?from=2026-11-09&to=2026-11-10"),
    )
    .await;
    assert_eq!(s, StatusCode::OK, "{r}");
    let list = r.as_array().unwrap();
    assert!(!list.is_empty() && list.len() <= 10);
    assert_eq!(list[0]["technician_id"], tina);
    assert!(list[0]["missing_skill_ids"].as_array().unwrap().is_empty());
    // 08:00 local is blocked until 12:00 plus the 15 minute buffer: first free is 13:00 (12:00Z).
    assert_eq!(list[0]["scheduled_start"], "2026-11-09T12:00:00Z");
    assert_eq!(list[0]["scheduled_end"], "2026-11-09T14:00:00Z");
    let tom = list.iter().find(|x| x["technician_id"] == TOM).unwrap();
    assert_eq!(tom["missing_skill_ids"], json!([skill]));
    assert_eq!(tom["scheduled_start"], "2026-11-09T07:00:00Z");
    // Same request, same answer.
    let (_, again) = get(
        &app,
        DISPATCHER,
        &format!("planning/suggestions/{job}?from=2026-11-09&to=2026-11-10"),
    )
    .await;
    assert_eq!(again, r);
    // A suggestion has no conflicts when scheduled.
    let (_, c) = schedule(
        &app,
        &job,
        "?dry_run=true",
        slot("2026-11-09T12:00:00Z", "2026-11-09T14:00:00Z", &[&tina]),
    )
    .await;
    assert!(c["conflicts"].as_array().unwrap().is_empty());
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn assigned_technicians_are_mailed_in_their_language_but_not_the_planner(db: PgPool) {
    let app = setup(db.clone()).await;
    office_hours(&app).await;
    sqlx::query("UPDATE core.users SET language = 'fr' WHERE email = 'technician@example.com'")
        .execute(&db)
        .await
        .unwrap();
    second_technician(&db, None).await; // company default language: nl
    let job = new_job(&app, "Service pump").await;

    // The planner assigns herself and two technicians: only the technicians are mailed.
    let (tina,): (String,) =
        sqlx::query_as("SELECT id::text FROM core.users WHERE email = 'tina@example.com'")
            .fetch_one(&db)
            .await
            .unwrap();
    let (s, r) = schedule(
        &app,
        &job,
        "",
        slot(
            "2026-11-09T08:00:00Z",
            "2026-11-09T10:00:00Z",
            &[TOM, &tina, DINA],
        ),
    )
    .await;
    assert_eq!(s, StatusCode::OK, "{r}");
    let tom_mail = mails(&db, "technician@example.com").await;
    assert_eq!(tom_mail.len(), 1);
    assert!(
        tom_mail[0].0.contains("vous est attribuée"),
        "{}",
        tom_mail[0].0
    );
    assert!(
        tom_mail[0].1.contains("2026-11-09 09:00 - 11:00"),
        "{}",
        tom_mail[0].1
    );
    assert!(tom_mail[0].1.contains("Alpha"));
    let tina_mail = mails(&db, "tina@example.com").await;
    assert_eq!(tina_mail.len(), 1);
    assert!(
        tina_mail[0].0.contains("is aan u toegewezen"),
        "{}",
        tina_mail[0].0
    );
    assert!(mails(&db, DISPATCHER).await.is_empty());

    // Same slot again: nobody is mailed. Moving it mails the kept technicians once more.
    schedule(
        &app,
        &job,
        "",
        slot(
            "2026-11-09T08:00:00Z",
            "2026-11-09T10:00:00Z",
            &[TOM, &tina],
        ),
    )
    .await;
    assert_eq!(mails(&db, "technician@example.com").await.len(), 1);
    schedule(
        &app,
        &job,
        "",
        slot(
            "2026-11-10T08:00:00Z",
            "2026-11-10T10:00:00Z",
            &[TOM, &tina],
        ),
    )
    .await;
    let tom_mail = mails(&db, "technician@example.com").await;
    assert_eq!(tom_mail.len(), 2);
    assert!(tom_mail[1].0.contains("déplacée"), "{}", tom_mail[1].0);
    assert!(mails(&db, "tina@example.com").await[1]
        .0
        .contains("verplaatst"));

    // All five languages have their own text.
    for (day, (lang, word)) in [
        ("16", ("en", "has been assigned")),
        ("17", ("de", "zugewiesen")),
        ("18", ("es", "asignado")),
    ] {
        sqlx::query("UPDATE core.users SET language = $1 WHERE email = 'technician@example.com'")
            .bind(lang)
            .execute(&db)
            .await
            .unwrap();
        let j = new_job(&app, "Other").await;
        let (s, _) = schedule(
            &app,
            &j,
            "",
            slot(
                &format!("2026-11-{day}T08:00:00Z"),
                &format!("2026-11-{day}T09:00:00Z"),
                &[TOM],
            ),
        )
        .await;
        assert_eq!(s, StatusCode::OK);
        let last = mails(&db, "technician@example.com").await.pop().unwrap();
        assert!(last.0.contains(word), "{lang}: {}", last.0);
    }
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn another_company_sees_and_changes_nothing(db: PgPool) {
    let app = setup(db.clone()).await;
    office_hours(&app).await;
    const OTHER: &str = "00000000-0000-7000-8000-0000000000b1";
    sqlx::query("INSERT INTO core.companies (id, name) VALUES ($1::uuid, 'Other BV')")
        .bind(OTHER)
        .execute(&db)
        .await
        .unwrap();
    sqlx::query("INSERT INTO core.users (id, email, display_name) VALUES (gen_random_uuid(), 'other@example.com', 'Other Admin')").execute(&db).await.unwrap();
    sqlx::query("INSERT INTO core.user_companies (user_id, company_id, roles, is_default) SELECT id, $1::uuid, '{admin}', true FROM core.users WHERE email = 'other@example.com'")
        .bind(OTHER).execute(&db).await.unwrap();
    const O: &str = "other@example.com";

    let job = new_job(&app, "Service").await;
    let (_, sk) = post(
        &app,
        ADMIN,
        "planning/skills",
        json!({ "code": "X", "name": "X" }),
    )
    .await;
    let skill = sk["id"].as_str().unwrap().to_string();
    let (_, ab) = post(&app, ADMIN, "planning/absences", json!({ "user_id": TOM, "from_date": "2026-11-10", "to_date": "2026-11-11", "kind": "sick" })).await;
    let absence = ab["id"].as_str().unwrap().to_string();

    // Nothing of the first company shows up.
    let (s, b) = get(&app, O, "planning/board?from=2026-11-09&to=2026-11-15").await;
    assert_eq!(s, StatusCode::OK);
    assert!(b["technicians"].as_array().unwrap().is_empty());
    assert!(b["backlog"].as_array().unwrap().is_empty());
    assert!(b["skills"].as_array().unwrap().is_empty());
    assert!(get(&app, O, "planning/skills")
        .await
        .1
        .as_array()
        .unwrap()
        .is_empty());
    assert!(get(&app, O, "planning/absences")
        .await
        .1
        .as_array()
        .unwrap()
        .is_empty());
    assert!(get(&app, O, "planning/working-hours")
        .await
        .1
        .as_array()
        .unwrap()
        .is_empty());

    // Nothing can be changed across companies.
    let body = slot("2026-11-09T08:00:00Z", "2026-11-09T10:00:00Z", &[]);
    assert_eq!(
        put(
            &app,
            O,
            &format!("planning/jobs/{job}/schedule"),
            body.clone()
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        put(
            &app,
            O,
            &format!("planning/jobs/{job}/schedule?dry_run=true"),
            body
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        get(&app, O, &format!("planning/suggestions/{job}")).await.0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        put(
            &app,
            O,
            &format!("planning/jobs/{job}/skills"),
            json!({ "skill_ids": [] })
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        put(
            &app,
            O,
            &format!("planning/skills/{skill}"),
            json!({ "code": "Y", "name": "Y" })
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        call(
            &app,
            O,
            Method::DELETE,
            &format!("/api/v1/planning/skills/{skill}"),
            None
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        call(
            &app,
            O,
            Method::DELETE,
            &format!("/api/v1/planning/absences/{absence}"),
            None
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    // Users and skills of the first company are unknown to the second.
    assert_eq!(
        put(
            &app,
            O,
            &format!("planning/users/{TOM}/skills"),
            json!({ "skills": [] })
        )
        .await
        .0,
        StatusCode::UNPROCESSABLE_ENTITY
    );
    assert_eq!(post(&app, O, "planning/absences", json!({ "user_id": TOM, "from_date": "2026-11-10", "to_date": "2026-11-11", "kind": "sick" })).await.0, StatusCode::UNPROCESSABLE_ENTITY);
    let (_, own) = post(
        &app,
        O,
        "planning/skills",
        json!({ "code": "Z", "name": "Z" }),
    )
    .await;
    assert_eq!(
        put(
            &app,
            ADMIN,
            &format!("planning/jobs/{job}/skills"),
            json!({ "skill_ids": [own["id"]] })
        )
        .await
        .0,
        StatusCode::UNPROCESSABLE_ENTITY
    );
    // The same code can be used in both companies.
    assert_eq!(
        post(
            &app,
            O,
            "planning/skills",
            json!({ "code": "X", "name": "X" })
        )
        .await
        .0,
        StatusCode::CREATED
    );
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn permissions_follow_the_roles(db: PgPool) {
    let app = setup(db.clone()).await;
    let job = new_job(&app, "Service").await;
    let body = slot("2026-11-09T08:00:00Z", "2026-11-09T10:00:00Z", &[TOM]);
    let board = "planning/board?from=2026-11-09&to=2026-11-15";

    // Technicians and sales do not see the board; a viewer reads it but changes nothing.
    for u in [TECH, SALES] {
        assert_eq!(get(&app, u, board).await.0, StatusCode::FORBIDDEN, "{u}");
        assert_eq!(
            schedule_as(&app, u, &job, body.clone()).await,
            StatusCode::FORBIDDEN,
            "{u}"
        );
    }
    assert_eq!(get(&app, VIEWER, board).await.0, StatusCode::OK);
    assert_eq!(
        schedule_as(&app, VIEWER, &job, body.clone()).await,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        get(&app, VIEWER, &format!("planning/suggestions/{job}"))
            .await
            .0,
        StatusCode::OK
    );

    // A dispatcher plans and records absences, but administration is for the admin.
    assert_eq!(get(&app, DISPATCHER, board).await.0, StatusCode::OK);
    assert_eq!(
        schedule_as(&app, DISPATCHER, &job, body).await,
        StatusCode::OK
    );
    let skill = json!({ "code": "A", "name": "A" });
    assert_eq!(
        post(&app, DISPATCHER, "planning/skills", skill.clone())
            .await
            .0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        post(&app, ADMIN, "planning/skills", skill).await.0,
        StatusCode::CREATED
    );
    assert_eq!(
        put(
            &app,
            DISPATCHER,
            "planning/working-hours",
            json!({ "slots": [] })
        )
        .await
        .0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        put(
            &app,
            DISPATCHER,
            "planning/settings",
            json!({ "travel_buffer_minutes": 5, "default_job_minutes": 60, "timezone": "UTC" })
        )
        .await
        .0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        put(
            &app,
            DISPATCHER,
            &format!("planning/users/{TOM}/skills"),
            json!({ "skills": [] })
        )
        .await
        .0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(post(&app, VIEWER, "planning/absences", json!({ "user_id": TOM, "from_date": "2026-11-10", "to_date": "2026-11-11", "kind": "sick" })).await.0, StatusCode::FORBIDDEN);
    assert_eq!(put(&app, ADMIN, "planning/settings", json!({ "travel_buffer_minutes": 5, "default_job_minutes": 60, "timezone": "Nowhere/Land" })).await.0, StatusCode::UNPROCESSABLE_ENTITY);

    // A technician reads their own absences, hours and skills, and nobody else's.
    post(&app, DISPATCHER, "planning/absences", json!({ "user_id": TOM, "from_date": "2026-11-10", "to_date": "2026-11-11", "kind": "training" })).await;
    let (s, mine) = get(&app, TECH, &format!("planning/absences?user_id={TOM}")).await;
    assert_eq!(s, StatusCode::OK);
    assert_eq!(mine.as_array().unwrap().len(), 1);
    assert_eq!(
        get(&app, TECH, "planning/absences").await.0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        get(&app, TECH, &format!("planning/absences?user_id={DINA}"))
            .await
            .0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        get(&app, TECH, &format!("planning/working-hours?user_id={TOM}"))
            .await
            .0,
        StatusCode::OK
    );
    assert_eq!(
        get(&app, TECH, "planning/working-hours?defaults=true")
            .await
            .0,
        StatusCode::OK
    );
    assert_eq!(
        get(&app, TECH, "planning/working-hours").await.0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        get(&app, TECH, &format!("planning/users/{TOM}/skills"))
            .await
            .0,
        StatusCode::OK
    );
    assert_eq!(
        get(&app, TECH, &format!("planning/users/{DINA}/skills"))
            .await
            .0,
        StatusCode::FORBIDDEN
    );
    // Technicians still see only their own jobs elsewhere.
    assert_eq!(
        get(&app, TECH, "field/my-jobs")
            .await
            .1
            .as_array()
            .unwrap()
            .len(),
        1
    );
}

async fn schedule_as(app: &axum::Router, user: &str, job: &str, body: Value) -> StatusCode {
    put(app, user, &format!("planning/jobs/{job}/schedule"), body)
        .await
        .0
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn working_hours_fall_back_to_the_company_default(db: PgPool) {
    let app = setup(db.clone()).await;
    // No hours at all: nothing is checked.
    let job = new_job(&app, "Night work").await;
    let late = slot("2026-11-09T21:00:00Z", "2026-11-09T22:00:00Z", &[TOM]);
    let (_, r) = schedule(&app, &job, "?dry_run=true", late.clone()).await;
    assert!(r["conflicts"].as_array().unwrap().is_empty());

    office_hours(&app).await;
    let (_, r) = schedule(&app, &job, "?dry_run=true", late).await;
    assert_eq!(kinds(&r), vec!["outside_hours"]);
    let (_, b) = get(
        &app,
        DISPATCHER,
        "planning/board?from=2026-11-09&to=2026-11-15",
    )
    .await;
    let tom = b["technicians"]
        .as_array()
        .unwrap()
        .iter()
        .find(|t| t["user_id"] == TOM)
        .unwrap();
    assert_eq!(tom["hours_are_default"], true);
    assert_eq!(tom["working_hours"].as_array().unwrap().len(), 10);

    // Tom's own hours replace the default entirely (Monday evenings only).
    let (s, rows) = put(
        &app,
        ADMIN,
        "planning/working-hours",
        json!({
            "user_id": TOM, "slots": [{ "weekday": 1, "start_time": "20:00", "end_time": "23:00" }]
        }),
    )
    .await;
    assert_eq!(s, StatusCode::OK, "{rows}");
    let (_, r) = schedule(
        &app,
        &job,
        "?dry_run=true",
        slot("2026-11-09T21:00:00Z", "2026-11-09T22:00:00Z", &[TOM]),
    )
    .await;
    assert!(r["conflicts"].as_array().unwrap().is_empty());
    let (_, r) = schedule(
        &app,
        &job,
        "?dry_run=true",
        slot("2026-11-10T08:00:00Z", "2026-11-10T09:00:00Z", &[TOM]),
    )
    .await;
    assert_eq!(kinds(&r), vec!["outside_hours"]);
    let (_, b) = get(
        &app,
        DISPATCHER,
        "planning/board?from=2026-11-09&to=2026-11-15",
    )
    .await;
    let tom = b["technicians"]
        .as_array()
        .unwrap()
        .iter()
        .find(|t| t["user_id"] == TOM)
        .unwrap();
    assert_eq!(tom["hours_are_default"], false);
    assert_eq!(tom["working_hours"].as_array().unwrap().len(), 1);

    // Removing his rows brings the default back; invalid or overlapping blocks are refused.
    put(
        &app,
        ADMIN,
        "planning/working-hours",
        json!({ "user_id": TOM, "slots": [] }),
    )
    .await;
    let (_, r) = schedule(
        &app,
        &job,
        "?dry_run=true",
        slot("2026-11-10T08:00:00Z", "2026-11-10T09:00:00Z", &[TOM]),
    )
    .await;
    assert!(r["conflicts"].as_array().unwrap().is_empty());
    for bad in [
        json!([{ "weekday": 0, "start_time": "08:00", "end_time": "09:00" }]),
        json!([{ "weekday": 1, "start_time": "09:00", "end_time": "09:00" }]),
        json!([{ "weekday": 1, "start_time": "08:00", "end_time": "10:00" }, { "weekday": 1, "start_time": "09:00", "end_time": "11:00" }]),
    ] {
        assert_eq!(
            put(
                &app,
                ADMIN,
                "planning/working-hours",
                json!({ "slots": bad })
            )
            .await
            .0,
            StatusCode::UNPROCESSABLE_ENTITY
        );
    }
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn board_backlog_lists_open_jobs_without_schedule_or_technician(db: PgPool) {
    let app = setup(db.clone()).await;
    let urgent = {
        let (_, j) = post(&app, DISPATCHER, "field/jobs", json!({ "title": "Urgent", "kind": "repair", "customer_id": ALPHA, "priority": "urgent" })).await;
        j["job"]["id"].as_str().unwrap().to_string()
    };
    let plain = new_job(&app, "Plain").await;
    // Scheduled but nobody assigned: still in the backlog.
    let half = new_job(&app, "Half").await;
    schedule(
        &app,
        &half,
        "",
        slot("2026-11-09T08:00:00Z", "2026-11-09T10:00:00Z", &[]),
    )
    .await;
    let placed = new_job(&app, "Placed").await;
    schedule(
        &app,
        &placed,
        "",
        slot("2026-11-09T08:00:00Z", "2026-11-09T10:00:00Z", &[TOM]),
    )
    .await;

    let (_, b) = get(
        &app,
        DISPATCHER,
        "planning/board?from=2026-11-09&to=2026-11-15",
    )
    .await;
    let ids: Vec<&str> = b["backlog"]
        .as_array()
        .unwrap()
        .iter()
        .map(|j| j["id"].as_str().unwrap())
        .collect();
    assert_eq!(ids[0], urgent);
    assert!(
        ids.contains(&plain.as_str())
            && ids.contains(&half.as_str())
            && !ids.contains(&placed.as_str())
    );
    assert_eq!(b["backlog"][0]["customer_name"], "Alpha Installaties NV");
    assert_eq!(b["jobs"].as_array().unwrap().len(), 1);
    assert_eq!(b["settings"]["travel_buffer_minutes"], 15);
    // Generated clients send dates as date-times; the day is what counts.
    let (s, _) = get(
        &app,
        DISPATCHER,
        "planning/board?from=2026-11-09T00:00:00.000&to=2026-11-15T00:00:00.000",
    )
    .await;
    assert_eq!(s, StatusCode::OK);
    // The period is validated.
    assert_eq!(
        get(
            &app,
            DISPATCHER,
            "planning/board?from=2026-11-15&to=2026-11-09"
        )
        .await
        .0,
        StatusCode::UNPROCESSABLE_ENTITY
    );
    assert_eq!(
        get(
            &app,
            DISPATCHER,
            "planning/board?from=2026-01-01&to=2026-12-31"
        )
        .await
        .0,
        StatusCode::UNPROCESSABLE_ENTITY
    );
}
