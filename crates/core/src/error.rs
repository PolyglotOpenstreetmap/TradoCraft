use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("not authenticated")]
    Unauthorized,
    #[error("{0}")]
    Forbidden(String),
    #[error("{0} not found")]
    NotFound(&'static str),
    #[error("{0}")]
    Validation(String),
    #[error("{0}")]
    Conflict(String),
    /// A conflict a client has to tell apart from other conflicts: `code` is a stable identifier.
    #[error("{1}")]
    ConflictCode(&'static str, String),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

/// Error body returned by every endpoint.
#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorBody {
    /// Machine-readable code: unauthorized, forbidden, not_found, validation, conflict, internal.
    pub error: &'static str,
    /// Stable identifier for errors a client must tell apart (for example `repair_status_changed`);
    /// absent for the rest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<&'static str>,
    pub message: String,
}

impl AppError {
    fn parts(&self) -> (StatusCode, &'static str) {
        match self {
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized"),
            AppError::Forbidden(_) => (StatusCode::FORBIDDEN, "forbidden"),
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, "not_found"),
            AppError::Validation(_) => (StatusCode::UNPROCESSABLE_ENTITY, "validation"),
            AppError::Conflict(_) | AppError::ConflictCode(..) => {
                (StatusCode::CONFLICT, "conflict")
            }
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal"),
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        if let sqlx::Error::Database(db) = &e {
            // 23505 unique_violation, 23503 foreign_key_violation, 23514 check_violation
            match db.code().as_deref() {
                Some("23505") => {
                    return AppError::Conflict("a record with this key already exists".into())
                }
                Some("23503") => {
                    return AppError::Validation("a referenced record does not exist".into())
                }
                Some("23514") => {
                    return AppError::Validation(format!("value rejected: {}", db.message()))
                }
                _ => {}
            }
        }
        AppError::Internal(e.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code) = self.parts();
        let detail = match &self {
            AppError::ConflictCode(c, _) => Some(*c),
            _ => None,
        };
        let message = match &self {
            AppError::Internal(e) => {
                tracing::error!(error = ?e, "internal error");
                "internal server error".to_string()
            }
            other => other.to_string(),
        };
        (
            status,
            Json(ErrorBody {
                error: code,
                code: detail,
                message,
            }),
        )
            .into_response()
    }
}
