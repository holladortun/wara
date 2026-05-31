use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};
use axum_valid::Valid;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    entities::environments::Environment,
    errors::ApiError,
    services::{auth::CurrentUser, projects::ProjectService},
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new().route(
        "/projects/{project_id}/environments",
        get(list_environments).post(create_environment),
    )
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct CreateEnvironmentRequest {
    #[validate(length(min = 1, max = 80))]
    pub name: String,
}

#[utoipa::path(get, path = "/api/v1/projects/{project_id}/environments", security(("bearer_auth" = [])), params(("project_id" = Uuid, Path)), responses((status = 200, body = [Environment])))]
pub async fn list_environments(
    _user: CurrentUser,
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
) -> Result<Json<Vec<Environment>>, ApiError> {
    Ok(Json(
        ProjectService::new(state.db)
            .list_environments(project_id)
            .await?,
    ))
}

#[utoipa::path(post, path = "/api/v1/projects/{project_id}/environments", security(("bearer_auth" = [])), params(("project_id" = Uuid, Path)), request_body = CreateEnvironmentRequest, responses((status = 200, body = Environment)))]
pub async fn create_environment(
    _user: CurrentUser,
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
    Valid(Json(payload)): Valid<Json<CreateEnvironmentRequest>>,
) -> Result<Json<Environment>, ApiError> {
    Ok(Json(
        ProjectService::new(state.db)
            .create_environment(project_id, payload.name)
            .await?,
    ))
}
