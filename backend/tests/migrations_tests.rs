use uuid::Uuid;
use wara_backend::{
    libs::{config::Config, db, migrations},
    services::projects::ProjectService,
};

/// Migrations applied to an empty database must yield a fully working schema
/// without ever calling `push_schema`, and the run must be idempotent.
#[tokio::test]
async fn migrations_initialize_a_fresh_database() {
    let Some(database_url) = Config::from_env().test_database_url else {
        eprintln!("skipping migration integration test; set WARA_TEST_DATABASE_URL to run it");
        return;
    };

    let test_database_url = create_isolated_database(&database_url).await;

    // First run applies the baseline.
    let report = migrations::run_pending(&test_database_url)
        .await
        .expect("apply migrations to fresh database");
    assert_eq!(report.applied, vec!["0001_baseline".to_string()]);
    assert_eq!(report.already_current, 0);
    assert!(!report.is_up_to_date());

    // Second run is a no-op: everything is already recorded.
    let report = migrations::run_pending(&test_database_url)
        .await
        .expect("re-run migrations");
    assert!(report.applied.is_empty());
    assert!(report.is_up_to_date());
    assert_eq!(report.already_current, 1);

    // The schema produced by migrations alone must be usable. Connect with both
    // push-schema and auto-migrate disabled so only the migrated schema is in play.
    let mut config = Config::from_env();
    config.database_url = test_database_url.clone();
    config.db_push_schema = false;
    config.db_auto_migrate = false;

    let database = db::connect(&config)
        .await
        .expect("connect to migrated database");
    let project_service = ProjectService::new(database);
    let project = project_service
        .create_project(
            format!("migration-check-{}", Uuid::now_v7().simple()),
            Some("created against a migrated schema".to_string()),
        )
        .await
        .expect("create project on migrated schema");
    let environments = project_service
        .list_environments(project.id)
        .await
        .expect("list environments on migrated schema");
    assert!(
        !environments.is_empty(),
        "project creation should seed a default environment"
    );

    drop_isolated_database(&test_database_url).await;
}

/// A database created the old way (Toasty `push_schema()`, no migration history)
/// must adopt the baseline cleanly rather than failing because its tables already
/// exist. This is the upgrade path for existing development databases.
#[tokio::test]
async fn migrations_adopt_an_existing_push_schema_database() {
    let Some(database_url) = Config::from_env().test_database_url else {
        eprintln!("skipping migration adoption test; set WARA_TEST_DATABASE_URL to run it");
        return;
    };

    let test_database_url = create_isolated_database(&database_url).await;

    // Reproduce a pre-migrations database: schema built by push_schema, with no
    // _wara_schema_migrations table.
    let mut config = Config::from_env();
    config.database_url = test_database_url.clone();
    config.db_push_schema = true;
    config.db_auto_migrate = false;
    db::connect(&config)
        .await
        .expect("build schema via push_schema");

    // Applying the baseline must adopt the existing schema, not error on
    // already-existing tables, and record the migration as applied.
    let report = migrations::run_pending(&test_database_url)
        .await
        .expect("adopt existing schema without error");
    assert_eq!(report.applied, vec!["0001_baseline".to_string()]);

    // And it stays idempotent afterwards.
    let report = migrations::run_pending(&test_database_url)
        .await
        .expect("re-run after adoption");
    assert!(report.is_up_to_date());

    // The adopted schema is still fully usable.
    config.db_push_schema = false;
    let database = db::connect(&config)
        .await
        .expect("connect to adopted database");
    let project = ProjectService::new(database)
        .create_project(format!("adoption-check-{}", Uuid::now_v7().simple()), None)
        .await
        .expect("create project on adopted schema");
    assert!(!project.name.is_empty());

    drop_isolated_database(&test_database_url).await;
}

/// A migration that was modified after being applied must fail loudly rather
/// than silently re-running or being ignored.
#[tokio::test]
async fn modified_applied_migration_is_rejected() {
    let Some(database_url) = Config::from_env().test_database_url else {
        eprintln!("skipping migration drift test; set WARA_TEST_DATABASE_URL to run it");
        return;
    };

    let test_database_url = create_isolated_database(&database_url).await;
    migrations::run_pending(&test_database_url)
        .await
        .expect("apply migrations");

    // Simulate the recorded checksum drifting from the embedded migration.
    let (client, connection) = tokio_postgres::connect(&test_database_url, tokio_postgres::NoTls)
        .await
        .expect("connect to tamper with history");
    tokio::spawn(async move {
        let _ = connection.await;
    });
    client
        .execute(
            "UPDATE _wara_schema_migrations SET checksum = 'tampered' WHERE version = '0001_baseline'",
            &[],
        )
        .await
        .expect("tamper checksum");
    drop(client);

    let error = migrations::run_pending(&test_database_url)
        .await
        .expect_err("modified migration must be rejected");
    let message = format!("{error:#}");
    assert!(
        message.contains("modified after being applied"),
        "unexpected error: {message}"
    );

    drop_isolated_database(&test_database_url).await;
}

async fn create_isolated_database(base_url: &str) -> String {
    let db_name = format!("wara_mig_test_{}", Uuid::now_v7().simple());
    let admin_url = replace_database_name(base_url, "postgres");
    let (client, connection) = tokio_postgres::connect(&admin_url, tokio_postgres::NoTls)
        .await
        .expect("connect postgres admin database");
    tokio::spawn(async move {
        if let Err(error) = connection.await {
            eprintln!("postgres admin connection error: {error}");
        }
    });
    client
        .execute(&format!(r#"CREATE DATABASE "{}""#, db_name), &[])
        .await
        .expect("create isolated test database");
    replace_database_name(base_url, &db_name)
}

async fn drop_isolated_database(database_url: &str) {
    let db_name = database_name(database_url);
    let admin_url = replace_database_name(database_url, "postgres");
    let (client, connection) = tokio_postgres::connect(&admin_url, tokio_postgres::NoTls)
        .await
        .expect("connect postgres admin database");
    tokio::spawn(async move {
        if let Err(error) = connection.await {
            eprintln!("postgres admin connection error: {error}");
        }
    });
    client
        .execute(
            "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = $1",
            &[&db_name],
        )
        .await
        .expect("terminate test database connections");
    client
        .execute(&format!(r#"DROP DATABASE IF EXISTS "{}""#, db_name), &[])
        .await
        .expect("drop isolated test database");
}

fn replace_database_name(database_url: &str, db_name: &str) -> String {
    let slash = database_url
        .rfind('/')
        .expect("database URL must contain a path");
    let query = database_url[slash + 1..]
        .find('?')
        .map(|index| slash + 1 + index);
    match query {
        Some(query) => format!(
            "{}{}{}",
            &database_url[..slash + 1],
            db_name,
            &database_url[query..]
        ),
        None => format!("{}{}", &database_url[..slash + 1], db_name),
    }
}

fn database_name(database_url: &str) -> String {
    let slash = database_url
        .rfind('/')
        .expect("database URL must contain a path");
    let value = &database_url[slash + 1..];
    value.split('?').next().unwrap_or(value).to_string()
}
