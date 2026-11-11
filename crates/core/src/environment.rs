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
    /// The word used in the API and in `APP_ENV_KIND`.
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

    /// Parses `live`, `test` or `tutorial` (any case, surrounding spaces allowed).
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

    /// Reads `APP_ENV_NAME` and `APP_ENV_KIND` from the process environment. A variable that is
    /// absent counts as unset (kind `live`); one that is present but not valid Unicode is an
    /// error, never silently the default.
    pub fn from_env() -> Result<Self, String> {
        Self::from_os_values(
            std::env::var_os("APP_ENV_NAME").as_deref(),
            std::env::var_os("APP_ENV_KIND").as_deref(),
        )
    }

    /// Like [`Self::from_values`] for raw environment values: `None` is an absent variable, a
    /// value that is not valid Unicode is an error naming the variable.
    pub fn from_os_values(
        name: Option<&std::ffi::OsStr>,
        kind: Option<&std::ffi::OsStr>,
    ) -> Result<Self, String> {
        fn text<'a>(var: &str, v: Option<&'a std::ffi::OsStr>) -> Result<Option<&'a str>, String> {
            match v {
                None => Ok(None),
                Some(os) => os
                    .to_str()
                    .map(Some)
                    .ok_or_else(|| format!("{var} is not valid UTF-8")),
            }
        }
        Self::from_values(text("APP_ENV_NAME", name)?, text("APP_ENV_KIND", kind)?)
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
/// Describes this instance: the configured name and kind, plus the database it is connected to.
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

    #[cfg(unix)]
    #[test]
    fn a_value_that_is_not_unicode_is_an_error() {
        use std::os::unix::ffi::OsStrExt;
        let bad = std::ffi::OsStr::from_bytes(&[0x74, 0xff, 0x73]);
        let e = EnvironmentConfig::from_os_values(None, Some(bad)).unwrap_err();
        assert!(e.contains("APP_ENV_KIND") && e.contains("UTF-8"), "{e}");
        let e = EnvironmentConfig::from_os_values(Some(bad), None).unwrap_err();
        assert!(e.contains("APP_ENV_NAME"), "{e}");
    }

    #[test]
    fn absent_os_values_are_the_defaults() {
        assert_eq!(
            EnvironmentConfig::from_os_values(None, None).unwrap(),
            EnvironmentConfig::default()
        );
    }

    #[test]
    fn an_unknown_kind_is_an_error() {
        let e = EnvironmentConfig::from_values(None, Some("staging")).unwrap_err();
        assert!(e.contains("APP_ENV_KIND") && e.contains("staging"), "{e}");
    }
}
