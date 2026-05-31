//! `wara-migrate` applies pending database schema migrations and exits.
//!
//! Run this before starting the backend in CI and production, or rely on the
//! backend's boot-time auto-migrate (`WARA_DB_AUTO_MIGRATE`, default on) for
//! local development.

use wara_backend::libs::{config::Config, migrations, telemetry};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env();
    let guard = telemetry::init(&config)?;

    let result = migrations::run_pending(&config.database_url).await;

    match &result {
        Ok(report) if report.is_up_to_date() => {
            tracing::info!(
                already_current = report.already_current,
                "database schema is up to date; no migrations to apply"
            );
        }
        Ok(report) => {
            tracing::info!(
                applied = ?report.applied,
                already_current = report.already_current,
                "applied database migrations"
            );
        }
        Err(error) => {
            tracing::error!(%error, "database migration failed");
        }
    }

    // Shut telemetry down on a best-effort basis: the migration outcome is the
    // meaningful exit status, so never let a shutdown error mask a migration error.
    let shutdown = telemetry::shutdown(guard);
    result.map(|_| ())?;
    shutdown
}
