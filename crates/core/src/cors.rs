//! Which browser origins may call a server (ADR 0050).
//!
//! Native clients (the Windows and mobile apps) send no `Origin` header and are not affected. A
//! browser client (the web build of the app, the customer portal) is only answered with CORS
//! headers when its origin is on the list.

use axum::http::HeaderValue;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CorsConfig {
    /// Every origin: local development and tests only.
    Any,
    /// Exactly these origins (`https://portal.example.com`); empty refuses every browser origin.
    Origins(Vec<String>),
}

impl CorsConfig {
    /// Parses a comma-separated list of origins; `*` alone means any origin.
    pub fn parse(list: &str) -> Result<Self, String> {
        let items: Vec<&str> = list
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .collect();
        if items == ["*"] {
            return Ok(CorsConfig::Any);
        }
        let mut origins = Vec::new();
        for item in items {
            // An origin has no path, query or fragment; one trailing slash is forgiven.
            let bare = item.strip_suffix('/').unwrap_or(item);
            let has_path = bare
                .split_once("://")
                .is_some_and(|(_, rest)| rest.contains(['/', '?', '#']));
            let origin = origin_of(bare).filter(|_| !has_path).ok_or_else(|| {
                format!("{item} is not an origin (expected e.g. https://portal.example.com)")
            })?;
            if !origins.contains(&origin) {
                origins.push(origin);
            }
        }
        Ok(CorsConfig::Origins(origins))
    }

    pub fn layer(&self) -> CorsLayer {
        let layer = CorsLayer::new().allow_methods(Any).allow_headers(Any);
        match self {
            CorsConfig::Any => layer.allow_origin(Any),
            CorsConfig::Origins(origins) => layer.allow_origin(AllowOrigin::list(
                origins.iter().filter_map(|o| HeaderValue::from_str(o).ok()),
            )),
        }
    }
}

/// The origin (`scheme://host[:port]`, lower case) of an http(s) URL, as a browser sends it in
/// the `Origin` header. `None` when the text is not such a URL.
pub fn origin_of(url: &str) -> Option<String> {
    let url = url.trim();
    let (scheme, rest) = url.split_once("://")?;
    let scheme = scheme.to_ascii_lowercase();
    if scheme != "http" && scheme != "https" {
        return None;
    }
    let authority = rest.split(['/', '?', '#']).next()?;
    let valid = !authority.is_empty()
        && authority
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | ':' | '[' | ']'));
    if !valid {
        return None;
    }
    let mut authority = authority.to_ascii_lowercase();
    // Browsers leave the default port out of the Origin header.
    let default_port = if scheme == "https" { ":443" } else { ":80" };
    if let Some(stripped) = authority.strip_suffix(default_port) {
        if !stripped.ends_with(':') && !stripped.is_empty() {
            authority = stripped.to_string();
        }
    }
    Some(format!("{scheme}://{authority}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origin_is_scheme_host_and_port() {
        assert_eq!(
            origin_of("https://Portal.Example.com/login#token=x").as_deref(),
            Some("https://portal.example.com")
        );
        assert_eq!(
            origin_of(" http://localhost:8081/ ").as_deref(),
            Some("http://localhost:8081")
        );
        assert_eq!(
            origin_of("https://portal.example.com:443").as_deref(),
            Some("https://portal.example.com")
        );
        assert_eq!(origin_of("portal.example.com"), None);
        assert_eq!(origin_of("ftp://example.com"), None);
        assert_eq!(origin_of("https://user@example.com"), None);
        assert_eq!(origin_of("https://"), None);
    }

    #[test]
    fn parses_a_list_and_refuses_what_is_not_an_origin() {
        assert_eq!(CorsConfig::parse("*"), Ok(CorsConfig::Any));
        assert_eq!(CorsConfig::parse(""), Ok(CorsConfig::Origins(vec![])));
        assert_eq!(
            CorsConfig::parse("https://a.example, http://localhost:8081/,https://a.example"),
            Ok(CorsConfig::Origins(vec![
                "https://a.example".into(),
                "http://localhost:8081".into()
            ]))
        );
        assert!(CorsConfig::parse("https://a.example/portal").is_err());
        assert!(CorsConfig::parse("a.example").is_err());
        assert!(CorsConfig::parse("*, https://a.example").is_err());
    }
}
