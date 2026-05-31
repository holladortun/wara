use crate::libs::config::Config;
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

    if config.db_push_schema {
        db.push_schema().await?;
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
