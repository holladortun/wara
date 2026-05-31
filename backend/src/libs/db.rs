use crate::libs::config::Config;
use crate::libs::migrations;
use crate::{
    entities::{
        credentials::{DockerCredentialRecord, EnvVarRecord},
        deployments::DeploymentRecord,
        domains::DomainRecord,
        environments::Environment,
        projects::Project,
        servers::ServerRecord,
        services::AppServiceRecord,
        templates::ProjectTemplateRecord,
        users::{UserInviteRecord, UserRecord},
    },
    errors::ApiError,
};

#[derive(Clone)]
pub struct Database {
    pub url: String,
    toasty: Option<toasty::Db>,
}

pub async fn connect(config: &Config) -> anyhow::Result<Database> {
    let db = toasty::Db::builder()
        .models(toasty::models!(
            Project,
            Environment,
            ServerRecord,
            AppServiceRecord,
            DockerCredentialRecord,
            EnvVarRecord,
            DomainRecord,
            DeploymentRecord,
            ProjectTemplateRecord,
            UserRecord,
            UserInviteRecord
        ))
        .connect(&config.database_url)
        .await?;

    // Schema management precedence:
    // - `WARA_DB_PUSH_SCHEMA` (default false) is an explicit dev escape hatch that
    //   lets Toasty regenerate the schema directly while iterating on models.
    // - Otherwise `WARA_DB_AUTO_MIGRATE` (default true) applies versioned
    //   migrations so fresh databases boot without any push, and upgrades are
    //   tracked. Migration failures propagate; there is no silent push fallback.
    // - If both are disabled, the operator is expected to run `wara-migrate`
    //   out of band, and queries fail loudly if the schema is absent.
    if config.db_push_schema {
        db.push_schema().await?;
    } else if config.db_auto_migrate {
        let report = migrations::run_pending(&config.database_url).await?;
        if report.is_up_to_date() {
            tracing::info!("database schema is up to date");
        } else {
            tracing::info!(applied = ?report.applied, "applied database migrations");
        }
    }

    tracing::info!("Toasty PostgreSQL database initialized");
    Ok(Database {
        url: config.database_url.clone(),
        toasty: Some(db),
    })
}

impl Database {
    pub fn unavailable_for_tests() -> Self {
        Self {
            url: "postgres://test-unavailable".to_string(),
            toasty: None,
        }
    }

    pub fn handle(&self) -> Result<toasty::Db, ApiError> {
        self.toasty
            .clone()
            .ok_or_else(|| ApiError::Internal("database handle is unavailable".to_string()))
    }
}
