//! Versioned SQL schema migrations.
//!
//! Toasty does not provide a migration workflow or expose its generated DDL, so
//! Wara owns an explicit, reviewable migration history. Migration files live in
//! `backend/migrations/` and are embedded into the binary at compile time, which
//! keeps `wara-migrate` and the backend self-contained in production images.
//!
//! The baseline migration is generated from Toasty itself (`db.push_schema()`
//! against a scratch database, dumped with `pg_dump --schema-only`) so the SQL
//! matches Toasty's runtime expectations exactly. Later schema changes are added
//! as new, additive `NNNN_*.sql` files.

use anyhow::{Context, Result, bail};
use sha2::{Digest, Sha256};
use tokio_postgres::NoTls;

/// Advisory lock key held while migrations run, so concurrent backend boots or
/// deploys serialize instead of racing on the same schema.
const ADVISORY_LOCK_KEY: i64 = 0x7761_7261_6d69_6701; // "waramig\x01"

struct Migration {
    version: &'static str,
    sql: &'static str,
}

/// Ordered list of all migrations. Append new entries; never edit an applied one
/// (the checksum drift guard will reject a modified, already-applied migration).
const MIGRATIONS: &[Migration] = &[Migration {
    version: "0001_baseline",
    sql: include_str!("../../migrations/0001_baseline.sql"),
}];

/// Summary of a migration run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationReport {
    /// Versions applied during this run, in order.
    pub applied: Vec<String>,
    /// Number of migrations that were already applied and left untouched.
    pub already_current: usize,
}

impl MigrationReport {
    pub fn is_up_to_date(&self) -> bool {
        self.applied.is_empty()
    }
}

fn checksum(sql: &str) -> String {
    let digest = Sha256::digest(sql.as_bytes());
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// Apply every migration that has not yet been recorded in the target database.
///
/// Each migration runs in its own transaction. A failure aborts the run and
/// returns an actionable error that names the offending version; the process
/// never silently falls back to `push_schema`.
pub async fn run_pending(database_url: &str) -> Result<MigrationReport> {
    let (client, connection) = tokio_postgres::connect(database_url, NoTls)
        .await
        .context("connect to database to run migrations")?;
    let connection_handle = tokio::spawn(async move {
        if let Err(error) = connection.await {
            tracing::error!(%error, "migration database connection error");
        }
    });

    let result = run_pending_with(&client).await;

    drop(client);
    connection_handle.abort();
    result
}

async fn run_pending_with(client: &tokio_postgres::Client) -> Result<MigrationReport> {
    client
        .batch_execute(&format!("SELECT pg_advisory_lock({ADVISORY_LOCK_KEY})"))
        .await
        .context("acquire migration advisory lock")?;

    let outcome = apply_all(client).await;

    // Always release the lock, even if a migration failed.
    let unlock = client
        .batch_execute(&format!("SELECT pg_advisory_unlock({ADVISORY_LOCK_KEY})"))
        .await
        .context("release migration advisory lock");

    let report = outcome?;
    unlock?;
    Ok(report)
}

async fn apply_all(client: &tokio_postgres::Client) -> Result<MigrationReport> {
    client
        .batch_execute(
            "CREATE TABLE IF NOT EXISTS _wara_schema_migrations (\
                version text PRIMARY KEY, \
                checksum text NOT NULL, \
                applied_at timestamptz NOT NULL DEFAULT now())",
        )
        .await
        .context("ensure migration history table exists")?;

    let applied_rows = client
        .query("SELECT version, checksum FROM _wara_schema_migrations", &[])
        .await
        .context("read applied migrations")?;

    let mut applied = Vec::new();
    let mut already_current = 0;

    for migration in MIGRATIONS {
        let expected = checksum(migration.sql);
        if let Some(row) = applied_rows
            .iter()
            .find(|row| row.get::<_, String>("version") == migration.version)
        {
            let recorded: String = row.get("checksum");
            if recorded != expected {
                bail!(
                    "migration {} was modified after being applied (recorded checksum {} != current {}); \
                     migrations must be append-only",
                    migration.version,
                    recorded,
                    expected
                );
            }
            already_current += 1;
            continue;
        }

        apply_one(client, migration, &expected)
            .await
            .with_context(|| format!("apply migration {}", migration.version))?;
        applied.push(migration.version.to_string());
    }

    Ok(MigrationReport {
        applied,
        already_current,
    })
}

async fn apply_one(
    client: &tokio_postgres::Client,
    migration: &Migration,
    checksum: &str,
) -> Result<()> {
    client.batch_execute("BEGIN").await.context("begin")?;

    let applied = async {
        client
            .batch_execute(migration.sql)
            .await
            .context("execute migration SQL")?;
        client
            .execute(
                "INSERT INTO _wara_schema_migrations (version, checksum) VALUES ($1, $2)",
                &[&migration.version, &checksum],
            )
            .await
            .context("record migration in history table")?;
        Ok::<(), anyhow::Error>(())
    }
    .await;

    match applied {
        Ok(()) => {
            client.batch_execute("COMMIT").await.context("commit")?;
            Ok(())
        }
        Err(error) => {
            // Best-effort rollback; surface the original error regardless.
            let _ = client.batch_execute("ROLLBACK").await;
            Err(error)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksum_is_stable_and_hex() {
        let value = checksum("SELECT 1;");
        assert_eq!(value.len(), 64);
        assert_eq!(value, checksum("SELECT 1;"));
        assert!(value.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn migration_versions_are_ordered_and_unique() {
        let mut previous: Option<&str> = None;
        for migration in MIGRATIONS {
            if let Some(prev) = previous {
                assert!(
                    prev < migration.version,
                    "migrations must be ordered and unique: {prev} >= {}",
                    migration.version
                );
            }
            previous = Some(migration.version);
        }
    }
}
