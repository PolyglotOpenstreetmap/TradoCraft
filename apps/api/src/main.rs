use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Context;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::EnvFilter;
use tradocraft_core::auth::{AuthConfig, OidcVerifier};
use tradocraft_core::{AppState, DocumentConfig};

fn env(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|v| !v.trim().is_empty())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // `tradocraft-api openapi` prints the OpenAPI document (used to generate the Dart client).
    if std::env::args().nth(1).as_deref() == Some("openapi") {
        println!(
            "{}",
            serde_json::to_string_pretty(&tradocraft_api::openapi())?
        );
        return Ok(());
    }

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| "info,sqlx=warn".into()),
        )
        .init();

    // `tradocraft-api migrate` applies the pending migrations and exits (used by `envctl clone`
    // so a clone is brought to this binary's schema before it is scrubbed and before any listener
    // starts).
    if std::env::args().nth(1).as_deref() == Some("migrate") {
        let url = env("DATABASE_URL").context("DATABASE_URL is required")?;
        let db = PgPoolOptions::new()
            .max_connections(2)
            .connect(&url)
            .await?;
        tradocraft_api::MIGRATOR
            .run(&db)
            .await
            .context("running migrations")?;
        tracing::info!("migrations applied");
        return Ok(());
    }

    // Fail fast on a bad APP_ENV_KIND, before touching the database.
    let environment = tradocraft_core::environment::EnvironmentConfig::from_env()
        .map_err(|e| anyhow::anyhow!(e))?;

    // The API only queues e-mail (the worker sends it) and never polls a mailbox; the banner makes
    // the policy visible for this instance too (ADR 0048).
    let requested_transport = env("MAIL_TRANSPORT").unwrap_or_else(|| "files".into());
    environment
        .outbound_policy(&requested_transport)
        .log_warnings(&requested_transport);

    let database_url = env("DATABASE_URL").context("DATABASE_URL is required")?;
    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await?;
    tradocraft_api::MIGRATOR
        .run(&db)
        .await
        .context("running migrations")?;

    let auth_mode = env("AUTH_MODE").unwrap_or_else(|| "oidc".into());
    let addr: SocketAddr = env("BIND_ADDR")
        .unwrap_or_else(|| "127.0.0.1:8080".into())
        .parse()
        .context("BIND_ADDR must look like 127.0.0.1:8080")?;

    // Development authentication trusts a client-supplied header, so it may only run where
    // nobody else can reach it: on loopback, or when explicitly acknowledged (e.g. a container
    // whose published port is bound to the host's loopback).
    if auth_mode == "dev" {
        let acknowledged = env("ALLOW_INSECURE_DEV_AUTH").as_deref() == Some("true");
        if !addr.ip().is_loopback() && !acknowledged {
            anyhow::bail!(
                "AUTH_MODE=dev refuses to listen on {addr}: it trusts the X-Dev-User header. \
                 Bind to 127.0.0.1, use AUTH_MODE=oidc, or set ALLOW_INSECURE_DEV_AUTH=true \
                 if the port is not reachable from other machines."
            );
        }
    }

    if env("SEED_DEV_DATA").as_deref() == Some("true") {
        anyhow::ensure!(
            auth_mode == "dev",
            "SEED_DEV_DATA=true creates well-known demo users and requires AUTH_MODE=dev"
        );
    }
    if env("SEED_DEV_DATA").as_deref() == Some("true") && tradocraft_api::seed_if_empty(&db).await?
    {
        tracing::info!("development seed data loaded");
        // Extra demo data kept outside the repository; only together with the development seed.
        if let Some(dir) = env("EXTRA_SEED_DIR") {
            let ran = tradocraft_api::load_extra_seed(&db, std::path::Path::new(&dir)).await?;
            tracing::info!(files = ?ran, "extra seed data loaded from {dir}");
        }
    }

    let auth = match auth_mode.as_str() {
        "dev" => {
            tracing::warn!("AUTH_MODE=dev: X-Dev-User header is trusted. Never use this on a reachable server.");
            AuthConfig::Dev {
                default_email: env("DEV_DEFAULT_USER"),
            }
        }
        "oidc" => AuthConfig::Oidc(OidcVerifier::new(
            env("OIDC_ISSUER").context("OIDC_ISSUER is required when AUTH_MODE=oidc")?,
            env("OIDC_AUDIENCE").context("OIDC_AUDIENCE is required when AUTH_MODE=oidc")?,
        )),
        other => anyhow::bail!("unknown AUTH_MODE {other} (expected oidc or dev)"),
    };

    let documents = DocumentConfig {
        typst_bin: PathBuf::from(env("TYPST_BIN").unwrap_or_else(|| "typst".into())),
        template_dir: PathBuf::from(env("TEMPLATE_DIR").unwrap_or_else(|| "templates".into())),
    };

    let state = AppState {
        db,
        auth: Arc::new(auth),
        documents: Arc::new(documents),
    };
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!(
        kind = environment.kind.as_str(),
        name = environment.name.as_deref().unwrap_or("(database name)"),
        "environment"
    );
    tracing::info!("listening on http://{addr} (API docs at /docs)");
    axum::serve(
        listener,
        tradocraft_api::app_with_environment(state, environment),
    )
    .with_graceful_shutdown(async {
        let _ = tokio::signal::ctrl_c().await;
    })
    .await?;
    Ok(())
}
