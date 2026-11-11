//! Which environment this API instance is: a name, a kind and the database it runs on.
//!
//! One database and one API instance exist per environment (ADR 0043). The app shows the
//! environment on every screen, so a user never mistakes a test or tutorial environment for live
//! data (or the other way round). The document is small and not sensitive, so it can be read
//! without signing in: the login screen shows it too.

use axum::extract::State;
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::{AppResult, AppState};

/// What an environment is for. Everything that reaches the outside world (e-mail, webhooks) may
/// only run for [`EnvKind::Live`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum EnvKind {
    /// Real company data.
    #[default]
    Live,
    /// A copy of live data, or scratch data, to try things out.
    Test,
    /// A practice environment for the guided tutorial.
    Tutorial,
}

impl EnvKind {
    pub fn as_str(self) -> &'static str {
        match self {
            EnvKind::Live => "live",
            EnvKind::Test => "test",
            EnvKind::Tutorial => "tutorial",
        }
    }
}

impl std::str::FromStr for EnvKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        match s.trim().to_ascii_lowercase().as_str() {
            "live" => Ok(EnvKind::Live),
            "test" => Ok(EnvKind::Test),
            "tutorial" => Ok(EnvKind::Tutorial),
            other => Err(format!(
                "unknown APP_ENV_KIND {other:?} (expected live, test or tutorial)"
            )),
        }
    }
}

/// Settings of this instance, read once at start-up (`APP_ENV_NAME`, `APP_ENV_KIND`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EnvironmentConfig {
    /// Display name; the database name when absent.
    pub name: Option<String>,
    pub kind: EnvKind,
}

impl EnvironmentConfig {
    /// Parses the two settings; an empty value counts as unset and an unknown kind is an error,
    /// so a typo cannot make a test instance look like live (or the reverse).
    pub fn from_values(name: Option<&str>, kind: Option<&str>) -> Result<Self, String> {
        fn non_empty(v: Option<&str>) -> Option<&str> {
            v.map(str::trim).filter(|v| !v.is_empty())
        }
        Ok(Self {
            name: non_empty(name).map(str::to_string),
            kind: match non_empty(kind) {
                Some(k) => k.parse()?,
                None => EnvKind::Live,
            },
        })
    }

    /// Reads `APP_ENV_NAME` and `APP_ENV_KIND` from the process environment.
    pub fn from_env() -> Result<Self, String> {
        Self::from_values(
            std::env::var("APP_ENV_NAME").ok().as_deref(),
            std::env::var("APP_ENV_KIND").ok().as_deref(),
        )
    }
}

/// The environment this API instance serves.
#[derive(Debug, Serialize, ToSchema)]
pub struct EnvironmentInfo {
    /// Display name (`APP_ENV_NAME`, default: the database name).
    pub name: String,
    pub kind: EnvKind,
    /// Name of the database only; never host, user or password.
    pub db_name: String,
    /// Version of the API.
    pub version: String,
}

#[utoipa::path(get, path = "/meta/environment", tag = "core", security(()),
    responses((status = 200, body = EnvironmentInfo)))]
async fn get_environment(
    State(state): State<AppState>,
    Extension(config): Extension<EnvironmentConfig>,
) -> AppResult<Json<EnvironmentInfo>> {
    let db_name: String = sqlx::query_scalar("SELECT current_database()")
        .fetch_one(&state.db)
        .await?;
    Ok(Json(EnvironmentInfo {
        name: config.name.clone().unwrap_or_else(|| db_name.clone()),
        kind: config.kind,
        db_name,
        version: env!("CARGO_PKG_VERSION").to_string(),
    }))
}

/// `GET /meta/environment`: no sign-in and no company needed.
pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(get_environment))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_live_without_a_name() {
        let c = EnvironmentConfig::from_values(None, None).unwrap();
        assert_eq!(c, EnvironmentConfig::default());
        assert_eq!(c.kind, EnvKind::Live);
        assert_eq!(c.name, None);
    }

    #[test]
    fn empty_values_count_as_unset() {
        let c = EnvironmentConfig::from_values(Some("  "), Some("")).unwrap();
        assert_eq!(c, EnvironmentConfig::default());
    }

    #[test]
    fn parses_every_kind_case_insensitively() {
        for (s, k) in [
            ("live", EnvKind::Live),
            ("TEST", EnvKind::Test),
            (" Tutorial ", EnvKind::Tutorial),
        ] {
            let c = EnvironmentConfig::from_values(Some("Acme"), Some(s)).unwrap();
            assert_eq!(c.kind, k);
            assert_eq!(c.name.as_deref(), Some("Acme"));
        }
    }

    #[test]
    fn an_unknown_kind_is_an_error() {
        let e = EnvironmentConfig::from_values(None, Some("staging")).unwrap_err();
        assert!(e.contains("APP_ENV_KIND") && e.contains("staging"), "{e}");
    }
}
