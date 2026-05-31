use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    entities::deployments::Deployment,
    errors::ApiError,
    services::{auth::CurrentUser, deployments::DeploymentService},
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/services/{service_id}/deployments",
            get(list_deployments).post(trigger_deploy),
        )
        .route("/deployments/{id}", get(get_deployment))
        .route("/services/{service_id}/restart", post(restart_service))
        .route("/services/{service_id}/logs", get(service_logs))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LogsResponse {
    pub output: String,
}

#[utoipa::path(get, path = "/api/v1/services/{service_id}/deployments", security(("bearer_auth" = [])), params(("service_id" = Uuid, Path)), responses((status = 200, body = [Deployment])))]
pub async fn list_deployments(
    _user: CurrentUser,
    State(state): State<AppState>,
    Path(service_id): Path<Uuid>,
) -> Result<Json<Vec<Deployment>>, ApiError> {
    Ok(Json(
        DeploymentService::new(state.db, state.config)
            .list_deployments(service_id)
            .await?,
    ))
}

#[utoipa::path(post, path = "/api/v1/services/{service_id}/deployments", security(("bearer_auth" = [])), params(("service_id" = Uuid, Path)), responses((status = 200, body = Deployment)))]
pub async fn trigger_deploy(
    _user: CurrentUser,
    State(state): State<AppState>,
    Path(service_id): Path<Uuid>,
) -> Result<Json<Deployment>, ApiError> {
    Ok(Json(
        DeploymentService::new(state.db, state.config)
            .trigger_deploy(service_id)
            .await?,
    ))
}

#[utoipa::path(get, path = "/api/v1/deployments/{id}", security(("bearer_auth" = [])), params(("id" = Uuid, Path)), responses((status = 200, body = Deployment)))]
pub async fn get_deployment(
    _user: CurrentUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Deployment>, ApiError> {
    Ok(Json(
        DeploymentService::new(state.db, state.config)
            .get_deployment(id)
            .await?,
    ))
}

#[utoipa::path(post, path = "/api/v1/services/{service_id}/restart", security(("bearer_auth" = [])), params(("service_id" = Uuid, Path)), responses((status = 200, body = Deployment)))]
pub async fn restart_service(
    _user: CurrentUser,
    State(state): State<AppState>,
    Path(service_id): Path<Uuid>,
) -> Result<Json<Deployment>, ApiError> {
    Ok(Json(
        DeploymentService::new(state.db, state.config)
            .restart_service(service_id)
            .await?,
    ))
}

#[utoipa::path(get, path = "/api/v1/services/{service_id}/logs", security(("bearer_auth" = [])), params(("service_id" = Uuid, Path)), responses((status = 200, body = LogsResponse)))]
pub async fn service_logs(
    _user: CurrentUser,
    State(state): State<AppState>,
    Path(service_id): Path<Uuid>,
) -> Result<Json<LogsResponse>, ApiError> {
    DeploymentService::new(state.db, state.config)
        .service_exists(service_id)
        .await?;
    Ok(Json(LogsResponse { output: "Platform-captured service log stream placeholder. Hosted app telemetry is not exported.".to_string() }))
}
