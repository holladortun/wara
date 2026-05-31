use toasty::stmt::{List, Query};
use uuid::Uuid;

use crate::{
    entities::credentials::{DockerCredentialRecord, EnvVarRecord},
    errors::ApiError,
    libs::{crypto, db::Database},
    routes::credentials::{CredentialResponse, EnvVarResponse},
    services::projects::ProjectService,
};

#[derive(Debug, Clone)]
pub struct CreateCredentialInput {
    pub project_id: Uuid,
    pub registry: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct CreateEnvVarInput {
    pub project_id: Uuid,
    pub environment_id: Option<Uuid>,
    pub service_id: Option<Uuid>,
    pub key: String,
    pub value: String,
}

#[derive(Clone)]
pub struct CredentialService {
    db: Database,
    secret_key: String,
}

impl CredentialService {
    pub fn new(db: Database, secret_key: String) -> Self {
        Self { db, secret_key }
    }

    pub async fn list_credentials(
        &self,
        project_id: Uuid,
    ) -> Result<Vec<CredentialResponse>, ApiError> {
        let mut db = self.db.handle()?;
        let records = Query::<List<DockerCredentialRecord>>::filter(
            DockerCredentialRecord::fields().project_id().eq(project_id),
        )
        .exec(&mut db)
        .await
        .map_err(map_toasty_error)?;
        Ok(records
            .into_iter()
            .map(|record| CredentialResponse {
                id: record.id,
                project_id: record.project_id,
                registry: record.registry,
                username: record.username,
                password: crypto::redact("secret"),
            })
            .collect())
    }

    pub async fn create_credential(
        &self,
        input: CreateCredentialInput,
    ) -> Result<CredentialResponse, ApiError> {
        ProjectService::new(self.db.clone())
            .get_project(input.project_id)
            .await?;
        let mut db = self.db.handle()?;
        let record = toasty::create!(DockerCredentialRecord {
            id: Uuid::now_v7(),
            project_id: input.project_id,
            registry: input.registry,
            username: input.username,
            encrypted_password: crypto::encrypt_secret(&self.secret_key, &input.password),
        })
        .exec(&mut db)
        .await
        .map_err(map_toasty_error)?;
        Ok(CredentialResponse {
            id: record.id,
            project_id: record.project_id,
            registry: record.registry,
            username: record.username,
            password: crypto::redact("secret"),
        })
    }

    pub async fn list_env_vars(&self, project_id: Uuid) -> Result<Vec<EnvVarResponse>, ApiError> {
        let mut db = self.db.handle()?;
        let records =
            Query::<List<EnvVarRecord>>::filter(EnvVarRecord::fields().project_id().eq(project_id))
                .exec(&mut db)
                .await
                .map_err(map_toasty_error)?;
        Ok(records.into_iter().map(env_var_response).collect())
    }

    pub async fn create_env_var(
        &self,
        input: CreateEnvVarInput,
    ) -> Result<EnvVarResponse, ApiError> {
        ProjectService::new(self.db.clone())
            .get_project(input.project_id)
            .await?;
        let mut db = self.db.handle()?;
        let record = toasty::create!(EnvVarRecord {
            id: Uuid::now_v7(),
            project_id: input.project_id,
            environment_id: input.environment_id,
            service_id: input.service_id,
            key: input.key,
            encrypted_value: crypto::encrypt_secret(&self.secret_key, &input.value),
        })
        .exec(&mut db)
        .await
        .map_err(map_toasty_error)?;
        Ok(env_var_response(record))
    }
}

fn env_var_response(record: EnvVarRecord) -> EnvVarResponse {
    EnvVarResponse {
        id: record.id,
        project_id: record.project_id,
        environment_id: record.environment_id,
        service_id: record.service_id,
        key: record.key,
        value: crypto::redact("secret"),
    }
}

fn map_toasty_error(error: toasty::Error) -> ApiError {
    ApiError::Internal(format!("database operation failed: {error}"))
}
