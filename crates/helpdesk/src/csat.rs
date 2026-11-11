//! Customer satisfaction: when a ticket is closed (and the company switched it on) the customer
//! gets an e-mail with two links, "good" and "bad". The links carry a random 256-bit token that
//! is stored hashed, works once and expires after 30 days. No portal login is needed: whoever
//! holds the link may rate (and add a comment), which is the point of a one-click survey.

use axum::extract::{Path, State};
use axum::Json;
use chrono::{DateTime, Duration, Utc};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::{FromRow, PgConnection};
use tradocraft_core::{audit, new_id, AppError, AppResult, AppState};
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use uuid::Uuid;

type ErrorBody = tradocraft_core::error::ErrorBody;

pub const TOKEN_PREFIX: &str = "tcq_";
pub const VALID_DAYS: i64 = 30;
const MAX_COMMENT: usize = 2000;

fn new_token() -> (String, Vec<u8>) {
    let mut bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut bytes);
    let token = format!(
        "{TOKEN_PREFIX}{}",
        bytes.iter().map(|b| format!("{b:02x}")).collect::<String>()
    );
    let hash = hash_token(&token);
    (token, hash)
}

pub fn hash_token(token: &str) -> Vec<u8> {
    Sha256::digest(token.trim().as_bytes()).to_vec()
}

/// Subject and body of the survey e-mail.
pub fn survey_mail(
    language: &str,
    number: &str,
    subject: &str,
    good_link: &str,
    bad_link: &str,
) -> (String, String) {
    let (title, intro, good, bad, valid) = match language {
        "nl" => (
            format!("Hoe tevreden bent u over ticket {number}?"),
            format!("Uw vraag {number} ({subject}) is afgehandeld. Hoe tevreden bent u over onze hulp?"),
            "Tevreden",
            "Niet tevreden",
            format!("De links zijn {VALID_DAYS} dagen geldig en kunnen één keer gebruikt worden."),
        ),
        "fr" => (
            format!("Êtes-vous satisfait du ticket {number} ?"),
            format!("Votre demande {number} ({subject}) est terminée. Êtes-vous satisfait de notre aide ?"),
            "Satisfait",
            "Pas satisfait",
            format!("Les liens sont valables {VALID_DAYS} jours et ne peuvent être utilisés qu'une seule fois."),
        ),
        "de" => (
            format!("Wie zufrieden sind Sie mit Ticket {number}?"),
            format!("Ihre Anfrage {number} ({subject}) wurde abgeschlossen. Wie zufrieden sind Sie mit unserer Hilfe?"),
            "Zufrieden",
            "Nicht zufrieden",
            format!("Die Links sind {VALID_DAYS} Tage gültig und können nur einmal verwendet werden."),
        ),
        "es" => (
            format!("¿Está satisfecho con el ticket {number}?"),
            format!("Su consulta {number} ({subject}) ha sido resuelta. ¿Está satisfecho con nuestra ayuda?"),
            "Satisfecho",
            "No satisfecho",
            format!("Los enlaces son válidos durante {VALID_DAYS} días y solo pueden usarse una vez."),
        ),
        _ => (
            format!("How did we do on ticket {number}?"),
            format!("Your request {number} ({subject}) has been closed. How satisfied are you with our help?"),
            "Satisfied",
            "Not satisfied",
            format!("The links are valid for {VALID_DAYS} days and can be used once."),
        ),
    };
    let body = format!("{intro}\n\n{good}: {good_link}\n{bad}: {bad_link}\n\n{valid}\n");
    (title, body)
}

/// Queues the survey e-mail for a ticket that was just closed. Does nothing unless the company
/// switched surveys on, the ticket is not internal, a recipient and the portal address are
/// known, and the ticket has not been asked before (one survey per ticket).
pub async fn queue_survey(conn: &mut PgConnection, ticket_id: Uuid) -> AppResult<()> {
    #[derive(FromRow)]
    struct T {
        company_id: Uuid,
        number: String,
        subject: String,
        language: String,
        recipient: Option<String>,
        portal_url: Option<String>,
        enabled: Option<bool>,
        already: bool,
    }
    let Some(t): Option<T> = sqlx::query_as(
        "SELECT t.company_id, t.number, t.subject, t.language::text AS language,
                coalesce(nullif(btrim(t.contact_email), ''), c.email) AS recipient,
                co.portal_url, s.csat_enabled AS enabled,
                EXISTS (SELECT 1 FROM helpdesk.csat_surveys x WHERE x.ticket_id = t.id) AS already
         FROM helpdesk.tickets t
         JOIN core.companies co ON co.id = t.company_id
         LEFT JOIN crm.customers c ON c.id = t.customer_id
         LEFT JOIN helpdesk.settings s ON s.company_id = t.company_id
         WHERE t.id = $1 AND NOT t.internal AND t.merged_into_id IS NULL",
    )
    .bind(ticket_id)
    .fetch_optional(&mut *conn)
    .await?
    else {
        return Ok(());
    };
    let (Some(true), false, Some(to), Some(portal)) = (
        t.enabled,
        t.already,
        t.recipient.filter(|r| r.contains('@')),
        t.portal_url.filter(|u| !u.trim().is_empty()),
    ) else {
        return Ok(());
    };
    let (token, hash) = new_token();
    sqlx::query(
        "INSERT INTO helpdesk.csat_surveys (id, company_id, ticket_id, token_hash, expires_at)
         VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(new_id())
    .bind(t.company_id)
    .bind(ticket_id)
    .bind(&hash)
    .bind(Utc::now() + Duration::days(VALID_DAYS))
    .execute(&mut *conn)
    .await?;
    let base = format!("{}/csat#token={token}", portal.trim_end_matches('/'));
    let (subject, body) = survey_mail(
        &t.language,
        &t.number,
        &t.subject,
        &format!("{base}&rating=good"),
        &format!("{base}&rating=bad"),
    );
    tradocraft_core::mail::enqueue(conn, t.company_id, &to, &subject, &body).await?;
    Ok(())
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CsatInfo {
    pub ticket_number: String,
    pub subject: String,
    pub company_name: String,
    /// The language of the ticket, for the page the customer lands on.
    pub language: String,
    /// open (can be rated) | rated | expired
    pub state: String,
    pub rating: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CsatInput {
    /// good | bad
    pub rating: String,
    #[serde(default)]
    pub comment: String,
}

#[derive(FromRow)]
struct SurveyRow {
    id: Uuid,
    ticket_id: Uuid,
    company_id: Uuid,
    ticket_number: String,
    subject: String,
    company_name: String,
    language: String,
    expires_at: DateTime<Utc>,
    rating: Option<String>,
}

async fn find(conn: &mut PgConnection, token: &str) -> AppResult<SurveyRow> {
    sqlx::query_as(
        "SELECT s.id, s.ticket_id, s.company_id, t.number AS ticket_number, t.subject, co.name AS company_name,
                t.language::text AS language, s.expires_at, s.rating
         FROM helpdesk.csat_surveys s JOIN helpdesk.tickets t ON t.id = s.ticket_id
         JOIN core.companies co ON co.id = s.company_id
         WHERE s.token_hash = $1",
    )
    .bind(hash_token(token))
    .fetch_optional(conn)
    .await?
    .ok_or(AppError::NotFound("survey"))
}

fn info(s: SurveyRow) -> CsatInfo {
    let state = if s.rating.is_some() {
        "rated"
    } else if s.expires_at <= Utc::now() {
        "expired"
    } else {
        "open"
    };
    CsatInfo {
        ticket_number: s.ticket_number,
        subject: s.subject,
        company_name: s.company_name,
        language: s.language,
        state: state.into(),
        rating: s.rating,
    }
}

/// What the customer sees before rating. Reading changes nothing, so mail scanners that open
/// links do not use up the token.
#[utoipa::path(get, path = "/csat/{token}", tag = "helpdesk", operation_id = "get_csat", params(("token" = String, Path)),
    responses((status = 200, body = CsatInfo), (status = 404, body = ErrorBody)))]
async fn get_survey(
    State(state): State<AppState>,
    Path(token): Path<String>,
) -> AppResult<Json<CsatInfo>> {
    let mut conn = state.db.acquire().await?;
    Ok(Json(info(find(&mut conn, &token).await?)))
}

/// Records the rating. The token works once and only until it expires.
#[utoipa::path(post, path = "/csat/{token}", tag = "helpdesk", operation_id = "rate_csat", params(("token" = String, Path)),
    request_body = CsatInput,
    responses((status = 200, body = CsatInfo), (status = 404, body = ErrorBody), (status = 409, body = ErrorBody), (status = 422, body = ErrorBody)))]
async fn rate(
    State(state): State<AppState>,
    Path(token): Path<String>,
    Json(input): Json<CsatInput>,
) -> AppResult<Json<CsatInfo>> {
    if !matches!(input.rating.as_str(), "good" | "bad") {
        return Err(AppError::Validation("rating must be good or bad".into()));
    }
    if input.comment.chars().count() > MAX_COMMENT {
        return Err(AppError::Validation(format!(
            "the comment is too long (at most {MAX_COMMENT} characters)"
        )));
    }
    let mut tx = state.db.begin().await?;
    let survey = find(&mut tx, &token).await?;
    let claimed = sqlx::query(
        "UPDATE helpdesk.csat_surveys SET rating = $2, comment = $3, rated_at = now()
         WHERE id = $1 AND rating IS NULL AND expires_at > now()",
    )
    .bind(survey.id)
    .bind(&input.rating)
    .bind(input.comment.trim())
    .execute(&mut *tx)
    .await?;
    if claimed.rows_affected() == 0 {
        return Err(AppError::Conflict(if survey.rating.is_some() {
            "this survey was already answered".into()
        } else {
            "this survey link has expired".into()
        }));
    }
    audit::record_system(
        &mut tx,
        survey.company_id,
        "ticket",
        survey.ticket_id,
        "csat_rated",
        serde_json::json!({ "rating": input.rating }),
    )
    .await?;
    let updated = find(&mut tx, &token).await?;
    tx.commit().await?;
    Ok(Json(info(updated)))
}

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(get_survey, rate))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_language_has_both_links_and_the_ticket() {
        for l in ["nl", "fr", "en", "de", "es"] {
            let (subject, body) =
                survey_mail(l, "T2026-00001", "Pump", "https://p/good", "https://p/bad");
            assert!(subject.contains("T2026-00001"), "{l}");
            assert!(
                body.contains("https://p/good") && body.contains("https://p/bad"),
                "{l}"
            );
            assert!(body.contains("30"), "{l}");
            assert!(
                !body.contains("[Ticket#"),
                "{l}: replies must not reopen the ticket"
            );
        }
    }

    #[test]
    fn tokens_are_long_random_and_hashed() {
        let (a, ha) = new_token();
        let (b, _) = new_token();
        assert_ne!(a, b);
        assert!(a.starts_with(TOKEN_PREFIX) && a.len() == TOKEN_PREFIX.len() + 64);
        assert_eq!(ha, hash_token(&a));
        assert_ne!(ha, a.as_bytes());
    }
}
