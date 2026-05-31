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
    entities::{projects::Project, templates::ProjectTemplate},
    errors::ApiError,
    services::{
        auth::CurrentUser,
        project_templates::{
            CreateProjectsFromTemplateInput, CreateTemplateInput, ProjectTemplateService,
        },
        templates::SecretCopyMode,
    },
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/projects/{project_id}/templates",
            get(list_project_templates).post(create_template),
        )
        .route(
            "/templates/{id}/projects",
            axum::routing::post(create_projects_from_template),
        )
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct CreateTemplateRequest {
    #[validate(length(min = 1, max = 120))]
    pub name: String,
    #[validate(length(max = 500))]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct CreateProjectsFromTemplateRequest {
    #[validate(length(min = 1, max = 100))]
    pub names: Vec<String>,
    pub secret_copy_mode: SecretCopyMode,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BulkProjectCreateResponse {
    pub projects: Vec<Project>,
}

#[utoipa::path(get, path = "/api/v1/projects/{project_id}/templates", security(("bearer_auth" = [])), params(("project_id" = Uuid, Path)), responses((status = 200, body = [ProjectTemplate])))]
pub async fn list_project_templates(
    _user: CurrentUser,
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
) -> Result<Json<Vec<ProjectTemplate>>, ApiError> {
    Ok(Json(
        ProjectTemplateService::new(state.db)
            .list_project_templates(project_id)
            .await?,
    ))
}

#[utoipa::path(post, path = "/api/v1/projects/{project_id}/templates", security(("bearer_auth" = [])), params(("project_id" = Uuid, Path)), request_body = CreateTemplateRequest, responses((status = 200, body = ProjectTemplate)))]
pub async fn create_template(
    _user: CurrentUser,
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
    Valid(Json(payload)): Valid<Json<CreateTemplateRequest>>,
) -> Result<Json<ProjectTemplate>, ApiError> {
    Ok(Json(
        ProjectTemplateService::new(state.db)
            .create_template(CreateTemplateInput {
                project_id,
                name: payload.name,
                description: payload.description,
            })
            .await?,
    ))
}

#[utoipa::path(post, path = "/api/v1/templates/{id}/projects", security(("bearer_auth" = [])), params(("id" = Uuid, Path)), request_body = CreateProjectsFromTemplateRequest, responses((status = 200, body = BulkProjectCreateResponse)))]
pub async fn create_projects_from_template(
    _user: CurrentUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Valid(Json(payload)): Valid<Json<CreateProjectsFromTemplateRequest>>,
) -> Result<Json<BulkProjectCreateResponse>, ApiError> {
    Ok(Json(BulkProjectCreateResponse {
        projects: ProjectTemplateService::new(state.db)
            .create_projects_from_template(CreateProjectsFromTemplateInput {
                template_id: id,
                names: payload.names,
                secret_copy_mode: payload.secret_copy_mode,
            })
            .await?,
    }))
}
