//! Knowledge base: entries, publishing, versions, visibility, search, suggestions, portal votes,
//! public reading, images, permissions and company isolation.

mod common;

use axum::body::Body;
use axum::http::{HeaderMap, Method, Request, StatusCode};
use common::{call, call_portal, portal_login, raw, setup, Who};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use sqlx::PgPool;
use tower::ServiceExt;

const COMPANY: &str = "00000000-0000-7000-8000-000000000001";
const ADMIN: &str = "admin@example.com";
const SUPPORT: &str = "support@example.com";
const SALES: &str = "sales@example.com";
const VIEWER: &str = "viewer@example.com";
const JAN: &str = "jan.peeters@alpha.example";
const SOFIE: &str = "sofie.maes@alpha.example";
const PUMP: &str = "00000000-0000-7000-8000-000000000303";
const MARK: char = '\u{E000}';

const PNG: &[u8] = &[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A, 0, 0, 0, 0];

fn text(lang: &str, title: &str, body: &str) -> Value {
    json!({ "language": lang, "title": title, "summary": "", "body": body })
}

async fn make(app: &axum::Router, user: &str, visibility: &str, texts: Vec<Value>) -> Value {
    let (status, e) = call(
        app,
        user,
        Method::POST,
        "/api/v1/kb/entries",
        Some(json!({ "visibility": visibility, "texts": texts })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{e}");
    e
}

fn id(e: &Value) -> String {
    e["id"].as_str().unwrap().to_string()
}

async fn publish(app: &axum::Router, entry: &str) -> Value {
    let (status, e) = call(
        app,
        ADMIN,
        Method::POST,
        &format!("/api/v1/kb/entries/{entry}/publish"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{e}");
    e
}

/// Creates and publishes an English entry.
async fn live(app: &axum::Router, visibility: &str, title: &str, body: &str) -> String {
    let e = make(app, SUPPORT, visibility, vec![text("en", title, body)]).await;
    publish(app, &id(&e)).await;
    id(&e)
}

async fn staff_list(app: &axum::Router, user: &str, query: &str) -> Vec<Value> {
    let (status, v) = call(
        app,
        user,
        Method::GET,
        &format!("/api/v1/kb/entries?{query}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{v}");
    v.as_array().unwrap().clone()
}

fn ids(list: &[Value]) -> Vec<String> {
    list.iter()
        .map(|e| e["id"].as_str().unwrap().to_string())
        .collect()
}

async fn portal_get(app: &axum::Router, token: &str, uri: &str) -> (StatusCode, Value) {
    call_portal(app, Some(token), Method::GET, uri, None).await
}

async fn public_get(app: &axum::Router, uri: &str) -> (StatusCode, HeaderMap, Value) {
    let res = app
        .clone()
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = res.status();
    let headers = res.headers().clone();
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    (
        status,
        headers,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn create_publish_version_and_restore(db: PgPool) {
    let app = setup(db.clone()).await;
    let e = make(
        &app,
        SUPPORT,
        "portal",
        vec![text(
            "en",
            "Reset the pump",
            "Hold the button for five seconds.",
        )],
    )
    .await;
    let eid = id(&e);
    assert_eq!(e["status"], "draft");
    assert_eq!(e["number"], "KB-00001");
    assert_eq!(e["slug"], "reset-the-pump");
    assert_eq!(e["version"], 0);
    assert_eq!(e["author_name"], "Support Agent");

    // Publishing is a separate right.
    let (status, _) = call(
        &app,
        SUPPORT,
        Method::POST,
        &format!("/api/v1/kb/entries/{eid}/publish"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    let p = publish(&app, &eid).await;
    assert_eq!(p["status"], "published");
    assert_eq!(p["version"], 1);
    assert!(p["published_at"].is_string());
    // Publishing twice is refused.
    let (status, _) = call(
        &app,
        ADMIN,
        Method::POST,
        &format!("/api/v1/kb/entries/{eid}/publish"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);

    // Editing the text of a published entry saves a version; changing only tags does not.
    let edit = |body: &str, tags: Value| json!({ "visibility": "portal", "texts": [text("en", "Reset the pump", body)], "tags": tags });
    let put = |b: Value| {
        let app = app.clone();
        let uri = format!("/api/v1/kb/entries/{eid}");
        async move { call(&app, SUPPORT, Method::PUT, &uri, Some(b)).await }
    };
    let (status, e2) = put(edit("Hold the button for ten seconds.", json!([]))).await;
    assert_eq!(status, StatusCode::OK, "{e2}");
    assert_eq!(e2["version"], 2);
    let (_, e3) = put(edit(
        "Hold the button for ten seconds.",
        json!(["Reset", "pump", "reset"]),
    ))
    .await;
    assert_eq!(e3["version"], 2);
    assert_eq!(e3["tags"], json!(["pump", "reset"]));

    // Changing the visibility of a published entry needs the right to publish.
    let (status, _) = call(
        &app,
        SUPPORT,
        Method::PUT,
        &format!("/api/v1/kb/entries/{eid}"),
        Some(json!({ "visibility": "public", "texts": [text("en", "Reset the pump", "x")] })),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);

    let (_, versions) = call(
        &app,
        SUPPORT,
        Method::GET,
        &format!("/api/v1/kb/entries/{eid}/versions"),
        None,
    )
    .await;
    let versions = versions.as_array().unwrap();
    assert_eq!(versions.len(), 2);
    assert_eq!(versions[0]["version"], 2);
    assert_eq!(versions[0]["editor_name"], "Support Agent");
    assert_eq!(
        versions[1]["texts"][0]["body"],
        "Hold the button for five seconds."
    );

    // Restoring writes a new version; history stays.
    let (status, r) = call(
        &app,
        SUPPORT,
        Method::POST,
        &format!("/api/v1/kb/entries/{eid}/versions/1/restore"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{r}");
    assert_eq!(r["version"], 3);
    assert_eq!(r["texts"][0]["body"], "Hold the button for five seconds.");
    let (_, versions) = call(
        &app,
        SUPPORT,
        Method::GET,
        &format!("/api/v1/kb/entries/{eid}/versions"),
        None,
    )
    .await;
    assert_eq!(versions.as_array().unwrap().len(), 3);
    assert_eq!(versions[0]["restored_from"], 1);
    let (status, _) = call(
        &app,
        SUPPORT,
        Method::POST,
        &format!("/api/v1/kb/entries/{eid}/versions/9/restore"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    // Versions are immutable in the database.
    let r = sqlx::query("UPDATE kb.entry_versions SET texts = '[]'::jsonb")
        .execute(&db)
        .await;
    assert!(r.is_err());

    // Unpublish, archive, and the numbers keep counting.
    let (_, u) = call(
        &app,
        ADMIN,
        Method::POST,
        &format!("/api/v1/kb/entries/{eid}/unpublish"),
        None,
    )
    .await;
    assert_eq!(u["status"], "draft");
    let (_, a) = call(
        &app,
        ADMIN,
        Method::POST,
        &format!("/api/v1/kb/entries/{eid}/archive"),
        None,
    )
    .await;
    assert_eq!(a["status"], "archived");
    let second = make(
        &app,
        SUPPORT,
        "internal",
        vec![text("en", "Reset the pump", "Again")],
    )
    .await;
    assert_eq!(second["number"], "KB-00002");
    assert_eq!(second["slug"], "reset-the-pump-2");

    // An entry without a body cannot be published.
    let empty = make(
        &app,
        SUPPORT,
        "internal",
        vec![text("en", "No body yet", "")],
    )
    .await;
    let (status, _) = call(
        &app,
        ADMIN,
        Method::POST,
        &format!("/api/v1/kb/entries/{}/publish", id(&empty)),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn visibility_matrix(db: PgPool) {
    let app = setup(db.clone()).await;
    let internal = live(&app, "internal", "Internal wiring notes", "Secret wiring").await;
    let portal = live(&app, "portal", "Portal wiring guide", "Customers wiring").await;
    let public = live(&app, "public", "Public wiring guide", "Everyone wiring").await;
    let draft = id(&make(
        &app,
        SUPPORT,
        "public",
        vec![text("en", "Draft wiring", "Wiring draft")],
    )
    .await);
    let archived = live(&app, "public", "Archived wiring", "Old wiring").await;
    call(
        &app,
        ADMIN,
        Method::POST,
        &format!("/api/v1/kb/entries/{archived}/archive"),
        None,
    )
    .await;

    // Staff with the right to read see everything, and can filter.
    let all = ids(&staff_list(&app, SALES, "").await);
    for e in [&internal, &portal, &public, &draft, &archived] {
        assert!(all.contains(e));
    }
    assert_eq!(
        ids(&staff_list(&app, SALES, "status=draft").await),
        vec![draft.clone()]
    );
    assert_eq!(
        ids(&staff_list(&app, SALES, "visibility=internal").await),
        vec![internal.clone()]
    );
    let (status, _) = call(
        &app,
        SALES,
        Method::GET,
        "/api/v1/kb/entries?status=bogus",
        None,
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);

    // A logged-in customer: published portal and public entries only.
    let jan = portal_login(&app, &db, JAN).await;
    let (status, list) = portal_get(&app, &jan, "/api/v1/portal/kb/entries?language=en").await;
    assert_eq!(status, StatusCode::OK, "{list}");
    let mut got = ids(list.as_array().unwrap());
    got.sort();
    let mut want = vec![portal.clone(), public.clone()];
    want.sort();
    assert_eq!(got, want);
    for hidden in [&internal, &draft, &archived] {
        let (status, _) =
            portal_get(&app, &jan, &format!("/api/v1/portal/kb/entries/{hidden}")).await;
        assert_eq!(status, StatusCode::NOT_FOUND, "{hidden}");
    }
    for shown in [&portal, &public] {
        let (status, _) =
            portal_get(&app, &jan, &format!("/api/v1/portal/kb/entries/{shown}")).await;
        assert_eq!(status, StatusCode::OK);
    }
    // Searching never leaks them either.
    let (_, found) = portal_get(&app, &jan, "/api/v1/portal/kb/entries?q=wiring&language=en").await;
    assert_eq!(found.as_array().unwrap().len(), 2);
    let (status, _) = call_portal(&app, None, Method::GET, "/api/v1/portal/kb/entries", None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    // The public: published public entries only, no login, cacheable.
    let (status, headers, list) = public_get(
        &app,
        &format!("/api/v1/public/kb/{COMPANY}/entries?language=en"),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(headers["cache-control"], "public, max-age=300");
    assert_eq!(ids(list.as_array().unwrap()), vec![public.clone()]);
    for hidden in [&internal, &portal, &draft, &archived] {
        let (status, _, _) = public_get(
            &app,
            &format!("/api/v1/public/kb/{COMPANY}/entries/{hidden}"),
        )
        .await;
        assert_eq!(status, StatusCode::NOT_FOUND, "{hidden}");
    }
    let (status, _, e) = public_get(
        &app,
        &format!("/api/v1/public/kb/{COMPANY}/entries/{public}"),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    // Nothing but what a reader needs: no counters, no status.
    for k in [
        "view_count",
        "helpful_yes",
        "status",
        "visibility",
        "author_id",
    ] {
        assert!(e.get(k).is_none(), "{k}");
    }

    // Unpublishing hides it again at once.
    call(
        &app,
        ADMIN,
        Method::POST,
        &format!("/api/v1/kb/entries/{public}/unpublish"),
        None,
    )
    .await;
    let (status, _, _) = public_get(
        &app,
        &format!("/api/v1/public/kb/{COMPANY}/entries/{public}"),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    // Reading the knowledge base needs a role that grants it.
    sqlx::query("INSERT INTO core.users (id, email, display_name) VALUES (gen_random_uuid(), 'tech@example.com', 'Tech')")
        .execute(&db).await.unwrap();
    sqlx::query("INSERT INTO core.user_companies (user_id, company_id, roles, is_default) SELECT id, $1::uuid, '{technician}', true FROM core.users WHERE email = 'tech@example.com'")
        .bind(COMPANY).execute(&db).await.unwrap();
    let (status, _) = call(
        &app,
        "tech@example.com",
        Method::GET,
        "/api/v1/kb/entries",
        None,
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn search_per_language_with_ranking_and_fallback(db: PgPool) {
    let app = setup(db.clone()).await;
    let both = make(
        &app,
        SUPPORT,
        "portal",
        vec![
            text(
                "nl",
                "Pomp onderhouden",
                "Smeer de lagers van de pomp elke maand.",
            ),
            text(
                "en",
                "Maintaining the pump",
                "Grease the bearings every month.",
            ),
        ],
    )
    .await;
    publish(&app, &id(&both)).await;
    let english_only = live(
        &app,
        "portal",
        "Leak troubleshooting",
        "If the seal drips, replace the gasket.",
    )
    .await;
    let title_hit = live(&app, "portal", "Gasket sizes", "A table of sizes.").await;
    let body_hit = live(
        &app,
        "portal",
        "Spare parts overview",
        "Order a gasket from the shop. Gasket gasket.",
    )
    .await;

    // Dutch: finds the Dutch text (stemming: "pompen" -> "pomp"), no fallback flag.
    let hits = staff_list(&app, SUPPORT, "q=pompen&language=nl").await;
    assert_eq!(ids(&hits), vec![id(&both)]);
    assert_eq!(hits[0]["language"], "nl");
    assert_eq!(hits[0]["fallback"], false);
    assert_eq!(hits[0]["title"], "Pomp onderhouden");
    assert!(
        hits[0]["snippet"].as_str().unwrap().contains(MARK),
        "{}",
        hits[0]["snippet"]
    );
    // An English word is not found in the Dutch text of that entry.
    assert!(staff_list(&app, SUPPORT, "q=bearings&language=nl")
        .await
        .is_empty());
    assert_eq!(
        ids(&staff_list(&app, SUPPORT, "q=bearings&language=en").await),
        vec![id(&both)]
    );

    // Entries without a Dutch text are searched in English and flagged.
    let hits = staff_list(&app, SUPPORT, "q=drips&language=nl").await;
    assert_eq!(ids(&hits), vec![english_only.clone()]);
    assert_eq!(hits[0]["language"], "en");
    assert_eq!(hits[0]["fallback"], true);

    // Ranking: a word in the title beats a word in the body, however often it is repeated.
    let hits = staff_list(&app, SUPPORT, "q=gasket&language=en").await;
    let order = ids(&hits);
    assert_eq!(order[0], title_hit);
    assert!(order.contains(&body_hit) && order.contains(&english_only));
    assert!(hits[0]["score"].as_f64().unwrap() > hits[1]["score"].as_f64().unwrap());

    // Phrase and exclusion syntax, filters.
    assert!(
        !ids(&staff_list(&app, SUPPORT, "q=gasket+-sizes&language=en").await).contains(&title_hit)
    );
    assert!(
        staff_list(&app, SUPPORT, "q=gasket&language=en&tag=nothing")
            .await
            .is_empty()
    );
    // Words that match nothing.
    assert!(staff_list(&app, SUPPORT, "q=zzzzqq").await.is_empty());
    // An unknown language is refused.
    let (status, _) = call(
        &app,
        SUPPORT,
        Method::GET,
        "/api/v1/kb/entries?language=xx",
        None,
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn categories_tags_and_related(db: PgPool) {
    let app = setup(db.clone()).await;
    let cat = |body: Value| {
        let app = app.clone();
        async move {
            call(
                &app,
                ADMIN,
                Method::POST,
                "/api/v1/kb/categories",
                Some(body),
            )
            .await
        }
    };
    let (status, parent) = cat(json!({ "names": [{ "language": "en", "name": "Pumps" }, { "language": "nl", "name": "Pompen" }] })).await;
    assert_eq!(status, StatusCode::CREATED, "{parent}");
    let pid = parent["id"].as_str().unwrap().to_string();
    let (status, child) =
        cat(json!({ "parent_id": pid, "names": [{ "language": "en", "name": "Seals" }] })).await;
    assert_eq!(status, StatusCode::CREATED, "{child}");
    let cid = child["id"].as_str().unwrap().to_string();
    // Only one level of nesting.
    let (status, _) =
        cat(json!({ "parent_id": cid, "names": [{ "language": "en", "name": "Too deep" }] })).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    // Categories are an admin matter.
    let (status, _) = call(
        &app,
        SUPPORT,
        Method::POST,
        "/api/v1/kb/categories",
        Some(json!({ "names": [{ "language": "en", "name": "X" }] })),
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);

    let (status, a) = call(&app, SUPPORT, Method::POST, "/api/v1/kb/entries", Some(json!({
        "visibility": "portal", "category_id": cid, "texts": [text("en", "Replace a seal", "Step one")],
        "tags": ["Seal"], "product_ids": [PUMP], "ticket_categories": ["service"]
    }))).await;
    assert_eq!(status, StatusCode::CREATED, "{a}");
    assert_eq!(a["product_ids"], json!([PUMP]));
    assert_eq!(a["ticket_categories"], json!(["service"]));
    publish(&app, &id(&a)).await;
    let b = make(
        &app,
        SUPPORT,
        "portal",
        vec![text("en", "Seal sizes", "Table")],
    )
    .await;
    publish(&app, &id(&b)).await;
    let (status, a2) = call(&app, SUPPORT, Method::PUT, &format!("/api/v1/kb/entries/{}", id(&a)), Some(json!({
        "visibility": "portal", "category_id": cid, "texts": [text("en", "Replace a seal", "Step one")],
        "tags": ["seal"], "related_ids": [id(&b), id(&a)]
    }))).await;
    assert_eq!(status, StatusCode::OK, "{a2}");
    assert_eq!(a2["related"].as_array().unwrap().len(), 1);
    assert_eq!(a2["related"][0]["title"], "Seal sizes");
    // A category from nowhere or a related entry from nowhere is refused.
    let (status, _) = call(
        &app,
        SUPPORT,
        Method::PUT,
        &format!("/api/v1/kb/entries/{}", id(&a)),
        Some(json!({
            "visibility": "portal", "texts": [text("en", "Replace a seal", "x")],
            "related_ids": ["00000000-0000-7000-8000-00000000ffff"]
        })),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);

    // Filtering by the parent category includes the child; the tag list feeds the filter.
    assert_eq!(
        ids(&staff_list(&app, SUPPORT, &format!("category_id={pid}")).await),
        vec![id(&a)]
    );
    assert_eq!(
        ids(&staff_list(&app, SUPPORT, "tag=seal").await),
        vec![id(&a)]
    );
    let (_, tags) = call(&app, SUPPORT, Method::GET, "/api/v1/kb/tags", None).await;
    assert_eq!(tags, json!(["seal"]));

    // The portal shows categories with readable entries, parent counts include children.
    let jan = portal_login(&app, &db, JAN).await;
    let (_, cats) = portal_get(&app, &jan, "/api/v1/portal/kb/categories?language=nl").await;
    let cats = cats.as_array().unwrap();
    assert_eq!(cats.len(), 2);
    let p = cats.iter().find(|c| c["id"] == pid.as_str()).unwrap();
    assert_eq!(p["name"], "Pompen");
    assert_eq!(p["entry_count"], 1);
    assert_eq!(
        cats.iter().find(|c| c["id"] == cid.as_str()).unwrap()["name"],
        "Seals"
    );
    let (_, in_parent) = portal_get(
        &app,
        &jan,
        &format!("/api/v1/portal/kb/entries?category_id={pid}"),
    )
    .await;
    assert_eq!(ids(in_parent.as_array().unwrap()), vec![id(&a)]);
    // The reader sees related entries that are readable.
    let (_, e) = portal_get(&app, &jan, &format!("/api/v1/portal/kb/entries/{}", id(&a))).await;
    assert_eq!(e["related"][0]["id"], id(&b).as_str());

    // A category in use cannot be deleted, an empty one can.
    let (status, _) = call(
        &app,
        ADMIN,
        Method::DELETE,
        &format!("/api/v1/kb/categories/{cid}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);
    let (_, empty) = cat(json!({ "names": [{ "language": "de", "name": "Leer" }] })).await;
    let (status, _) = call(
        &app,
        ADMIN,
        Method::DELETE,
        &format!("/api/v1/kb/categories/{}", empty["id"].as_str().unwrap()),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NO_CONTENT);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn suggestions_for_a_ticket(db: PgPool) {
    let app = setup(db.clone()).await;
    let seal = live(
        &app,
        "internal",
        "Replacing a leaking seal",
        "Remove the cover and fit a new gasket.",
    )
    .await;
    let portal = live(
        &app,
        "portal",
        "Pump keeps dripping",
        "Check the gasket first.",
    )
    .await;
    let _unrelated = live(
        &app,
        "portal",
        "Invoices explained",
        "Where to find your invoices.",
    )
    .await;
    let _draft = make(
        &app,
        SUPPORT,
        "portal",
        vec![text("en", "Dripping draft", "gasket dripping")],
    )
    .await;

    // The customer writes in through the portal: subject plus the first message.
    let jan = portal_login(&app, &db, JAN).await;
    let (status, t) = call_portal(
        &app,
        Some(&jan),
        Method::POST,
        "/api/v1/portal/tickets",
        Some(json!({
            "subject": "Problem", "body": "The pump is dripping and the gasket looks old"
        })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{t}");
    let ticket = t["ticket"]["id"].as_str().unwrap().to_string();
    let language: String =
        sqlx::query_scalar("SELECT language::text FROM helpdesk.tickets WHERE id = $1::uuid")
            .bind(&ticket)
            .fetch_one(&db)
            .await
            .unwrap();

    let (status, s) = call(
        &app,
        SUPPORT,
        Method::GET,
        &format!("/api/v1/tickets/{ticket}/kb-suggestions"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{s}");
    let got = ids(s.as_array().unwrap());
    // The ticket may be in another language than English: entries then come via the fallback.
    assert!(
        got.contains(&seal) && got.contains(&portal),
        "{language}: {s}"
    );
    assert_eq!(got.len(), 2, "{s}");
    assert!(s[0]["snippet"].as_str().unwrap().contains(MARK));

    // Render in the ticket's language: plain text and a portal link, none for internal entries.
    let (status, r) = call(
        &app,
        SUPPORT,
        Method::GET,
        &format!("/api/v1/tickets/{ticket}/kb-entries/{portal}/render"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{r}");
    assert_eq!(r["title"], "Pump keeps dripping");
    assert_eq!(r["text"], "Check the gasket first.");
    assert_eq!(
        r["link"],
        format!("http://localhost:8081/help/{portal}").as_str()
    );
    let (_, r) = call(
        &app,
        SUPPORT,
        Method::GET,
        &format!("/api/v1/tickets/{ticket}/kb-entries/{seal}/render"),
        None,
    )
    .await;
    assert!(r["link"].is_null());
    // Drafts cannot be inserted into a reply.
    let (_, drafts) = call(
        &app,
        SUPPORT,
        Method::GET,
        "/api/v1/kb/entries?status=draft",
        None,
    )
    .await;
    let d = id(&drafts[0]);
    let (status, _) = call(
        &app,
        SUPPORT,
        Method::GET,
        &format!("/api/v1/tickets/{ticket}/kb-entries/{d}/render"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    // A ticket of nobody's is a 404, and a ticket of words that are all too short gives nothing.
    let (status, _) = call(
        &app,
        SUPPORT,
        Method::GET,
        "/api/v1/tickets/00000000-0000-7000-8000-00000000ffff/kb-suggestions",
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    // The portal suggests while the contact types a subject: at most three, readable ones only.
    let (status, s) = portal_get(
        &app,
        &jan,
        "/api/v1/portal/kb/suggestions?q=pump+dripping+gasket&language=en",
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(ids(s.as_array().unwrap()), vec![portal.clone()]);
    let (_, none) = portal_get(&app, &jan, "/api/v1/portal/kb/suggestions?q=ab").await;
    assert!(none.as_array().unwrap().is_empty());
    for i in 0..4 {
        live(&app, "public", &format!("Gasket variant {i}"), "gasket").await;
    }
    let (_, s) = portal_get(
        &app,
        &jan,
        "/api/v1/portal/kb/suggestions?q=gasket&language=en",
    )
    .await;
    assert_eq!(s.as_array().unwrap().len(), 3);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn company_isolation(db: PgPool) {
    let app = setup(db.clone()).await;
    const OTHER: &str = "00000000-0000-7000-8000-0000000000b1";
    sqlx::query("INSERT INTO core.companies (id, name) VALUES ($1::uuid, 'Other BV')")
        .bind(OTHER)
        .execute(&db)
        .await
        .unwrap();
    sqlx::query("INSERT INTO core.users (id, email, display_name) VALUES (gen_random_uuid(), 'other@example.com', 'Other Admin')").execute(&db).await.unwrap();
    sqlx::query("INSERT INTO core.user_companies (user_id, company_id, roles, is_default) SELECT id, $1::uuid, '{admin}', true FROM core.users WHERE email = 'other@example.com'")
        .bind(OTHER).execute(&db).await.unwrap();

    let theirs = make(
        &app,
        "other@example.com",
        "public",
        vec![text("en", "Their secret manual", "unique-zebra content")],
    )
    .await;
    publish_as(&app, "other@example.com", &id(&theirs)).await;
    assert_eq!(theirs["number"], "KB-00001");
    let mine = live(&app, "public", "Our manual", "unique-zebra too").await;

    let uri = format!("/api/v1/kb/entries/{}", id(&theirs));
    for method in [Method::GET, Method::PUT] {
        let body = (method == Method::PUT)
            .then(|| json!({ "visibility": "internal", "texts": [text("en", "x", "y")] }));
        let (status, _) = call(&app, ADMIN, method, &uri, body).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
    }
    for action in ["publish", "unpublish", "archive"] {
        let (status, _) = call(&app, ADMIN, Method::POST, &format!("{uri}/{action}"), None).await;
        assert_eq!(status, StatusCode::NOT_FOUND, "{action}");
    }
    let (status, _) = call(&app, ADMIN, Method::GET, &format!("{uri}/versions"), None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(
        ids(&staff_list(&app, ADMIN, "q=unique-zebra").await),
        vec![mine.clone()]
    );
    assert_eq!(
        staff_list(&app, "other@example.com", "q=zebra").await.len(),
        1
    );

    // Related entries cannot cross the border.
    let (status, _) = call(&app, ADMIN, Method::PUT, &format!("/api/v1/kb/entries/{mine}"), Some(json!({
        "visibility": "public", "texts": [text("en", "Our manual", "z")], "related_ids": [id(&theirs)]
    }))).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);

    // Customers and the public only see their own company.
    let jan = portal_login(&app, &db, JAN).await;
    let (status, _) = portal_get(
        &app,
        &jan,
        &format!("/api/v1/portal/kb/entries/{}", id(&theirs)),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    let (_, found) = portal_get(&app, &jan, "/api/v1/portal/kb/entries?q=zebra&language=en").await;
    assert_eq!(ids(found.as_array().unwrap()), vec![mine.clone()]);
    let (status, _, _) = public_get(
        &app,
        &format!("/api/v1/public/kb/{COMPANY}/entries/{}", id(&theirs)),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    let (status, _, _) = public_get(
        &app,
        &format!("/api/v1/public/kb/{OTHER}/entries/{}", id(&theirs)),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let (_, _, list) =
        public_get(&app, &format!("/api/v1/public/kb/{OTHER}/entries?q=zebra")).await;
    assert_eq!(ids(list.as_array().unwrap()), vec![id(&theirs)]);
    let (status, _, list) = public_get(
        &app,
        "/api/v1/public/kb/00000000-0000-7000-8000-0000000000ff/entries",
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(list.as_array().unwrap().is_empty());
    // A vote cannot cross either.
    let (status, _) = call_portal(
        &app,
        Some(&jan),
        Method::PUT,
        &format!("/api/v1/portal/kb/entries/{}/vote", id(&theirs)),
        Some(json!({ "helpful": true })),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

async fn get(app: &axum::Router, who: Who<'_>, uri: String) -> (StatusCode, HeaderMap, Vec<u8>) {
    raw(app, who, Method::GET, &uri, None).await
}

async fn publish_as(app: &axum::Router, user: &str, entry: &str) {
    let (status, e) = call(
        app,
        user,
        Method::POST,
        &format!("/api/v1/kb/entries/{entry}/publish"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{e}");
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn helpful_votes_and_views(db: PgPool) {
    let app = setup(db.clone()).await;
    let portal = live(
        &app,
        "portal",
        "How to log in",
        "Use the link in your e-mail.",
    )
    .await;
    let internal = live(&app, "internal", "Staff only", "x").await;
    let jan = portal_login(&app, &db, JAN).await;
    let sofie = portal_login(&app, &db, SOFIE).await;
    let vote = |token: String, entry: String, helpful: bool| {
        let app = app.clone();
        async move {
            call_portal(
                &app,
                Some(&token),
                Method::PUT,
                &format!("/api/v1/portal/kb/entries/{entry}/vote"),
                Some(json!({ "helpful": helpful })),
            )
            .await
            .0
        }
    };
    let counters = || {
        let app = app.clone();
        let portal = portal.clone();
        async move {
            let (_, e) = call(
                &app,
                SUPPORT,
                Method::GET,
                &format!("/api/v1/kb/entries/{portal}"),
                None,
            )
            .await;
            (
                e["helpful_yes"].as_i64().unwrap(),
                e["helpful_no"].as_i64().unwrap(),
                e["view_count"].as_i64().unwrap(),
            )
        }
    };

    assert_eq!(
        vote(jan.clone(), portal.clone(), true).await,
        StatusCode::NO_CONTENT
    );
    assert_eq!(counters().await, (1, 0, 0));
    // The same vote again counts once.
    assert_eq!(
        vote(jan.clone(), portal.clone(), true).await,
        StatusCode::NO_CONTENT
    );
    assert_eq!(counters().await, (1, 0, 0));
    // A changed mind moves the vote.
    assert_eq!(
        vote(jan.clone(), portal.clone(), false).await,
        StatusCode::NO_CONTENT
    );
    assert_eq!(counters().await, (0, 1, 0));
    assert_eq!(
        vote(sofie.clone(), portal.clone(), true).await,
        StatusCode::NO_CONTENT
    );
    assert_eq!(counters().await, (1, 1, 0));
    let rows: i64 = sqlx::query_scalar("SELECT count(*) FROM kb.votes")
        .fetch_one(&db)
        .await
        .unwrap();
    assert_eq!(rows, 2);
    // Internal entries cannot be voted on; no login, no vote.
    assert_eq!(
        vote(jan.clone(), internal.clone(), true).await,
        StatusCode::NOT_FOUND
    );
    let (status, _) = call_portal(
        &app,
        None,
        Method::PUT,
        &format!("/api/v1/portal/kb/entries/{portal}/vote"),
        Some(json!({ "helpful": true })),
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    // Reading shows the contact's own vote and counts one view per request.
    let (_, e) = portal_get(&app, &jan, &format!("/api/v1/portal/kb/entries/{portal}")).await;
    assert_eq!(e["my_vote"], false);
    portal_get(&app, &sofie, &format!("/api/v1/portal/kb/entries/{portal}")).await;
    assert_eq!(counters().await.2, 2);
    // Staff reading does not count as a view; hidden entries are never counted.
    portal_get(&app, &jan, &format!("/api/v1/portal/kb/entries/{internal}")).await;
    let (_, i) = call(
        &app,
        SUPPORT,
        Method::GET,
        &format!("/api/v1/kb/entries/{internal}"),
        None,
    )
    .await;
    assert_eq!(i["view_count"], 0);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn images_follow_the_visibility_of_the_entry(db: PgPool) {
    let app = setup(db.clone()).await;
    let upload = |user: &'static str, entry: String, bytes: Vec<u8>| {
        let app = app.clone();
        async move {
            let (status, _, body) = raw(
                &app,
                Who::Staff(user),
                Method::POST,
                &format!("/api/v1/kb/entries/{entry}/images?filename=a.png"),
                Some(("application/octet-stream", bytes)),
            )
            .await;
            (
                status,
                serde_json::from_slice::<Value>(&body).unwrap_or(Value::Null),
            )
        }
    };
    let portal = live(&app, "portal", "With picture", "![the part](image:x)").await;
    let public = live(&app, "public", "Public picture", "pic").await;
    let internal = live(&app, "internal", "Internal picture", "pic").await;

    // Only JPEG, PNG and WebP are accepted: not a PDF, SVG, GIF or text, and not an empty file.
    for bad in [
        b"%PDF-1.4 data".to_vec(),
        b"<svg xmlns='http://www.w3.org/2000/svg'/>".to_vec(),
        b"GIF89a....".to_vec(),
        b"hello".to_vec(),
        vec![],
    ] {
        let (status, _) = upload("support@example.com", portal.clone(), bad).await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    }
    let (status, _) = upload("sales@example.com", portal.clone(), PNG.to_vec()).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    let mut files = Vec::new();
    for e in [&portal, &public, &internal] {
        let (status, f) = upload("support@example.com", e.clone(), PNG.to_vec()).await;
        assert_eq!(status, StatusCode::CREATED, "{f}");
        assert_eq!(f["content_type"], "image/png");
        files.push(f["id"].as_str().unwrap().to_string());
    }
    let (_, e) = call(
        &app,
        SUPPORT,
        Method::GET,
        &format!("/api/v1/kb/entries/{portal}"),
        None,
    )
    .await;
    assert_eq!(e["images"].as_array().unwrap().len(), 1);

    let jan = portal_login(&app, &db, JAN).await;
    // Staff.
    let (status, h, body) = get(
        &app,
        Who::Staff(SALES),
        format!("/api/v1/kb/entries/{internal}/images/{}", files[2]),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(h["content-type"], "image/png");
    assert_eq!(h["x-content-type-options"], "nosniff");
    assert_eq!(body, PNG);
    // Customers: portal and public entries, not internal ones, and not through another entry.
    let (status, _, _) = get(
        &app,
        Who::Portal(&jan),
        format!("/api/v1/portal/kb/entries/{portal}/images/{}", files[0]),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let (status, _, _) = get(
        &app,
        Who::Portal(&jan),
        format!("/api/v1/portal/kb/entries/{public}/images/{}", files[1]),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let (status, _, _) = get(
        &app,
        Who::Portal(&jan),
        format!("/api/v1/portal/kb/entries/{internal}/images/{}", files[2]),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    let (status, _, _) = get(
        &app,
        Who::Portal(&jan),
        format!("/api/v1/portal/kb/entries/{portal}/images/{}", files[1]),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    // The public: public entries only.
    let (status, h, _) = get(
        &app,
        Who::Portal("none"),
        format!(
            "/api/v1/public/kb/{COMPANY}/entries/{public}/images/{}",
            files[1]
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(h["cache-control"], "public, max-age=300");
    for (e, f) in [(&portal, &files[0]), (&internal, &files[2])] {
        let (status, _, _) = get(
            &app,
            Who::Portal("none"),
            format!("/api/v1/public/kb/{COMPANY}/entries/{e}/images/{f}"),
        )
        .await;
        assert_eq!(status, StatusCode::NOT_FOUND);
    }
    // Unpublishing the entry hides its pictures too.
    call(
        &app,
        ADMIN,
        Method::POST,
        &format!("/api/v1/kb/entries/{public}/unpublish"),
        None,
    )
    .await;
    let (status, _, _) = get(
        &app,
        Who::Portal("none"),
        format!(
            "/api/v1/public/kb/{COMPANY}/entries/{public}/images/{}",
            files[1]
        ),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    // Deleting removes the picture.
    let (status, _) = call(
        &app,
        SUPPORT,
        Method::DELETE,
        &format!("/api/v1/kb/entries/{portal}/images/{}", files[0]),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    let (status, _, _) = get(
        &app,
        Who::Staff(SALES),
        format!("/api/v1/kb/entries/{portal}/images/{}", files[0]),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[sqlx::test(migrator = "tradocraft_api::MIGRATOR")]
async fn permissions(db: PgPool) {
    let app = setup(db.clone()).await;
    let e = make(
        &app,
        SUPPORT,
        "internal",
        vec![text("en", "Hello", "World")],
    )
    .await;
    let eid = id(&e);
    // Read-only roles read but cannot write.
    for user in [SALES, VIEWER] {
        let (status, _) = call(
            &app,
            user,
            Method::GET,
            &format!("/api/v1/kb/entries/{eid}"),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{user}");
        let (status, _) = call(
            &app,
            user,
            Method::POST,
            "/api/v1/kb/entries",
            Some(json!({ "visibility": "internal", "texts": [text("en", "x", "y")] })),
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN, "{user}");
        let (status, _) = call(
            &app,
            user,
            Method::PUT,
            &format!("/api/v1/kb/entries/{eid}"),
            Some(json!({ "visibility": "internal", "texts": [text("en", "x", "y")] })),
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN, "{user}");
        let (status, _) = call(
            &app,
            user,
            Method::POST,
            &format!("/api/v1/kb/entries/{eid}/publish"),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN, "{user}");
    }
    let (status, me) = call(&app, SUPPORT, Method::GET, "/api/v1/me", None).await;
    assert_eq!(status, StatusCode::OK);
    let perms = me["permissions"].as_array().unwrap();
    assert!(
        perms.contains(&json!("kb.read"))
            && perms.contains(&json!("kb.write"))
            && !perms.contains(&json!("kb.admin"))
    );
    // The public needs no login and sees an empty list, not an error.
    let (status, _, list) = public_get(&app, &format!("/api/v1/public/kb/{COMPANY}/entries")).await;
    assert_eq!(status, StatusCode::OK);
    assert!(list.as_array().unwrap().is_empty());
    // Bad input.
    let (status, _) = call(
        &app,
        SUPPORT,
        Method::POST,
        "/api/v1/kb/entries",
        Some(json!({ "visibility": "secret", "texts": [text("en", "x", "y")] })),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    let (status, _) = call(
        &app,
        SUPPORT,
        Method::POST,
        "/api/v1/kb/entries",
        Some(json!({ "visibility": "internal", "texts": [] })),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
}
