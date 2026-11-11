//! Portal tokens and the extractor that turns `Authorization: Bearer <session token>` into the
//! logged-in contact.
//!
//! Login links and session tokens are 256 random bits. Only their SHA-256 hash is stored, so a
//! database leak does not give access. Both are prefixed so that leaked ones are recognisable.

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use rand::RngCore;
use sha2::{Digest, Sha256};
use tradocraft_core::{AppError, AppState};
use tradocraft_helpdesk::portal::Viewer;
use uuid::Uuid;

pub const LINK_PREFIX: &str = "tcl_";
pub const SESSION_PREFIX: &str = "tcs_";

/// A new random token and its hash.
pub fn new_token(prefix: &str) -> (String, Vec<u8>) {
    let mut bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut bytes);
    let token = format!(
        "{prefix}{}",
        bytes.iter().map(|b| format!("{b:02x}")).collect::<String>()
    );
    let hash = hash_token(&token);
    (token, hash)
}

pub fn hash_token(token: &str) -> Vec<u8> {
    Sha256::digest(token.trim().as_bytes()).to_vec()
}

/// The contact behind a valid portal session.
#[derive(Debug, Clone)]
pub struct PortalUser {
    pub session_id: Uuid,
    pub contact_id: Uuid,
    pub customer_id: Uuid,
    pub company_id: Uuid,
    pub scope: String,
}

impl PortalUser {
    pub fn viewer(&self) -> Viewer {
        Viewer {
            company_id: self.company_id,
            contact_id: self.contact_id,
            customer_id: self.customer_id,
            scope: self.scope.clone(),
        }
    }
}

impl FromRequestParts<AppState> for PortalUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "))
            .map(str::trim)
            .filter(|t| t.starts_with(SESSION_PREFIX))
            .ok_or(AppError::Unauthorized)?;
        // The contact must still be allowed in: switching off portal access, deactivating the
        // person or the customer ends the session immediately.
        let row: Option<(Uuid, Uuid, Uuid, Uuid, String)> = sqlx::query_as(
            "SELECT s.id, s.contact_id, c.customer_id, s.company_id, c.portal_scope
             FROM portal.sessions s
             JOIN crm.contacts c ON c.id = s.contact_id
             JOIN crm.customers cu ON cu.id = c.customer_id
             WHERE s.token_hash = $1 AND s.revoked_at IS NULL AND s.expires_at > now()
               AND c.active AND c.portal_access AND cu.active",
        )
        .bind(hash_token(token))
        .fetch_optional(&state.db)
        .await?;
        let (session_id, contact_id, customer_id, company_id, scope) =
            row.ok_or(AppError::Unauthorized)?;
        Ok(PortalUser {
            session_id,
            contact_id,
            customer_id,
            company_id,
            scope,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokens_are_unique_prefixed_and_hash_stably() {
        let (a, ha) = new_token(SESSION_PREFIX);
        let (b, _) = new_token(SESSION_PREFIX);
        assert_ne!(a, b);
        assert!(a.starts_with(SESSION_PREFIX));
        assert_eq!(a.len(), SESSION_PREFIX.len() + 64);
        assert_eq!(hash_token(&a), ha);
        assert_eq!(hash_token(&format!(" {a}\n")), ha);
    }
}
