//! Mail to technicians when a job is assigned to them or moved, in their own language (the
//! company's default when they have none), queued in the caller's transaction.

use chrono::NaiveDateTime;
use sqlx::{FromRow, PgConnection};
use tradocraft_core::mail;
use uuid::Uuid;

/// What the mail is about.
pub struct JobMail<'a> {
    pub number: &'a str,
    pub title: &'a str,
    pub customer: &'a str,
    pub city: &'a str,
    /// Local start and end.
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Assigned,
    Rescheduled,
}

#[derive(FromRow)]
struct Recipient {
    email: String,
    language: String,
}

/// (subject, intro line, "when", "customer", "place") per language. `{n}` is the job number and
/// `{t}` its title.
fn text(lang: &str, kind: Kind) -> [&'static str; 5] {
    match (lang, kind) {
        ("nl", Kind::Assigned) => [
            "Opdracht {n} is aan u toegewezen",
            "Opdracht {n} ({t}) is aan u toegewezen.",
            "Wanneer",
            "Klant",
            "Plaats",
        ],
        ("nl", Kind::Rescheduled) => [
            "Opdracht {n} is verplaatst",
            "Het tijdstip van opdracht {n} ({t}) is gewijzigd.",
            "Wanneer",
            "Klant",
            "Plaats",
        ],
        ("fr", Kind::Assigned) => [
            "La mission {n} vous est attribuée",
            "La mission {n} ({t}) vous a été attribuée.",
            "Quand",
            "Client",
            "Lieu",
        ],
        ("fr", Kind::Rescheduled) => [
            "La mission {n} a été déplacée",
            "L'horaire de la mission {n} ({t}) a changé.",
            "Quand",
            "Client",
            "Lieu",
        ],
        ("de", Kind::Assigned) => [
            "Auftrag {n} wurde Ihnen zugewiesen",
            "Auftrag {n} ({t}) wurde Ihnen zugewiesen.",
            "Wann",
            "Kunde",
            "Ort",
        ],
        ("de", Kind::Rescheduled) => [
            "Auftrag {n} wurde verschoben",
            "Der Termin von Auftrag {n} ({t}) hat sich geändert.",
            "Wann",
            "Kunde",
            "Ort",
        ],
        ("es", Kind::Assigned) => [
            "Se le ha asignado el trabajo {n}",
            "Se le ha asignado el trabajo {n} ({t}).",
            "Cuándo",
            "Cliente",
            "Lugar",
        ],
        ("es", Kind::Rescheduled) => [
            "El trabajo {n} se ha reprogramado",
            "Ha cambiado la hora del trabajo {n} ({t}).",
            "Cuándo",
            "Cliente",
            "Lugar",
        ],
        (_, Kind::Assigned) => [
            "Job {n} has been assigned to you",
            "Job {n} ({t}) has been assigned to you.",
            "When",
            "Customer",
            "Place",
        ],
        (_, Kind::Rescheduled) => [
            "Job {n} has been rescheduled",
            "The appointment of job {n} ({t}) has changed.",
            "When",
            "Customer",
            "Place",
        ],
    }
}

fn fill(s: &str, job: &JobMail) -> String {
    s.replace("{n}", job.number).replace("{t}", job.title)
}

/// Queues one mail per active recipient among `ids`.
pub async fn send(
    conn: &mut PgConnection,
    company_id: Uuid,
    ids: &[Uuid],
    kind: Kind,
    job: &JobMail<'_>,
) -> Result<(), sqlx::Error> {
    if ids.is_empty() {
        return Ok(());
    }
    let recipients: Vec<Recipient> = sqlx::query_as(
        "SELECT u.email, coalesce(u.language, c.default_language)::text AS language
         FROM core.users u
         JOIN core.user_companies uc ON uc.user_id = u.id AND uc.company_id = $1
         JOIN core.companies c ON c.id = uc.company_id
         WHERE u.id = ANY($2) AND u.active ORDER BY u.email",
    )
    .bind(company_id)
    .bind(ids)
    .fetch_all(&mut *conn)
    .await?;
    let when = format!(
        "{} - {}",
        job.start.format("%Y-%m-%d %H:%M"),
        job.end.format("%H:%M")
    );
    for r in recipients {
        let [subject, intro, l_when, l_customer, l_place] = text(&r.language, kind);
        let mut body = format!(
            "{}\n\n{l_when}: {when}\n{l_customer}: {}\n",
            fill(intro, job),
            job.customer
        );
        if !job.city.is_empty() {
            body.push_str(&format!("{l_place}: {}\n", job.city));
        }
        mail::enqueue(conn, company_id, &r.email, &fill(subject, job), &body).await?;
    }
    Ok(())
}
