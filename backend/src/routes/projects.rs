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
    entities::projects::Project,
    errors::ApiError,
    services::{auth::CurrentUser, projects::ProjectService},
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/projects", get(list_projects).post(create_project))
        .route("/projects/{id}", get(get_project))
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct CreateProjectRequest {
    #[validate(length(min = 1, max = 120))]
    pub name: String,
    #[validate(length(max = 500))]
    pub description: Option<String>,
}

#[utoipa::path(get, path = "/api/v1/projects", security(("bearer_auth" = [])), responses((status = 200, body = [Project])))]
pub async fn list_projects(
    _user: CurrentUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<Project>>, ApiError> {
    Ok(Json(ProjectService::new(state.db).list_projects().await?))
}

#[utoipa::path(post, path = "/api/v1/projects", security(("bearer_auth" = [])), request_body = CreateProjectRequest, responses((status = 200, body = Project)))]
pub async fn create_project(
    _user: CurrentUser,
    State(state): State<AppState>,
    Valid(Json(payload)): Valid<Json<CreateProjectRequest>>,
) -> Result<Json<Project>, ApiError> {
    Ok(Json(
        ProjectService::new(state.db)
            .create_project(payload.name, payload.description)
            .await?,
    ))
}

#[utoipa::path(get, path = "/api/v1/projects/{id}", security(("bearer_auth" = [])), params(("id" = Uuid, Path)), responses((status = 200, body = Project)))]
pub async fn get_project(
    _user: CurrentUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Project>, ApiError> {
    Ok(Json(ProjectService::new(state.db).get_project(id).await?))
}
