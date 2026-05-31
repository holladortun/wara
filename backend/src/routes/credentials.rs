use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};
use axum_valid::Valid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    services::{
        auth::CurrentUser,
        credentials::{CreateCredentialInput, CreateEnvVarInput, CredentialService},
    },
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/projects/{project_id}/credentials",
            get(list_credentials).post(create_credential),
        )
        .route(
            "/projects/{project_id}/env-vars",
            get(list_env_vars).post(create_env_var),
        )
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct CreateCredentialRequest {
    #[validate(length(min = 1, max = 253))]
    pub registry: String,
    #[validate(length(min = 1))]
    pub username: String,
    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CredentialResponse {
    pub id: Uuid,
    pub project_id: Uuid,
    pub registry: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct CreateEnvVarRequest {
    pub environment_id: Option<Uuid>,
    pub service_id: Option<Uuid>,
    #[validate(length(min = 1, max = 255))]
    pub key: String,
    #[validate(length(min = 1))]
    pub value: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EnvVarResponse {
    pub id: Uuid,
    pub project_id: Uuid,
    pub environment_id: Option<Uuid>,
    pub service_id: Option<Uuid>,
    pub key: String,
    pub value: String,
}

#[utoipa::path(get, path = "/api/v1/projects/{project_id}/credentials", security(("bearer_auth" = [])), params(("project_id" = Uuid, Path)), responses((status = 200, body = [CredentialResponse])))]
pub async fn list_credentials(
    _user: CurrentUser,
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
) -> Result<Json<Vec<CredentialResponse>>, crate::errors::ApiError> {
    Ok(Json(
        CredentialService::new(state.db, state.config.secret_key)
            .list_credentials(project_id)
            .await?,
    ))
}

#[utoipa::path(post, path = "/api/v1/projects/{project_id}/credentials", security(("bearer_auth" = [])), params(("project_id" = Uuid, Path)), request_body = CreateCredentialRequest, responses((status = 200, body = CredentialResponse)))]
pub async fn create_credential(
    _user: CurrentUser,
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
    Valid(Json(payload)): Valid<Json<CreateCredentialRequest>>,
) -> Result<Json<CredentialResponse>, crate::errors::ApiError> {
    Ok(Json(
        CredentialService::new(state.db, state.config.secret_key)
            .create_credential(CreateCredentialInput {
                project_id,
                registry: payload.registry,
                username: payload.username,
                password: payload.password,
            })
            .await?,
    ))
}

#[utoipa::path(get, path = "/api/v1/projects/{project_id}/env-vars", security(("bearer_auth" = [])), params(("project_id" = Uuid, Path)), responses((status = 200, body = [EnvVarResponse])))]
pub async fn list_env_vars(
    _user: CurrentUser,
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
) -> Result<Json<Vec<EnvVarResponse>>, crate::errors::ApiError> {
    Ok(Json(
        CredentialService::new(state.db, state.config.secret_key)
            .list_env_vars(project_id)
            .await?,
    ))
}

#[utoipa::path(post, path = "/api/v1/projects/{project_id}/env-vars", security(("bearer_auth" = [])), params(("project_id" = Uuid, Path)), request_body = CreateEnvVarRequest, responses((status = 200, body = EnvVarResponse)))]
pub async fn create_env_var(
    _user: CurrentUser,
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
    Valid(Json(payload)): Valid<Json<CreateEnvVarRequest>>,
) -> Result<Json<EnvVarResponse>, crate::errors::ApiError> {
    Ok(Json(
        CredentialService::new(state.db, state.config.secret_key)
            .create_env_var(CreateEnvVarInput {
                project_id,
                environment_id: payload.environment_id,
                service_id: payload.service_id,
                key: payload.key,
                value: payload.value,
            })
            .await?,
    ))
}
