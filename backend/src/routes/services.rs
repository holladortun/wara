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
    entities::services::AppService,
    errors::ApiError,
    libs::docker::DeployKind,
    services::{
        app_services::{AppServiceService, CreateAppServiceInput},
        auth::CurrentUser,
    },
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/projects/{project_id}/services",
            get(list_services).post(create_service),
        )
        .route("/services/{id}", get(get_service))
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct CreateServiceRequest {
    pub environment_id: Uuid,
    #[validate(length(min = 1, max = 120))]
    pub name: String,
    pub deploy_kind: DeployKind,
    #[validate(length(min = 1))]
    pub image: Option<String>,
    #[validate(length(min = 1))]
    pub compose_file: Option<String>,
    #[validate(length(min = 1))]
    pub dockerfile: Option<String>,
    #[validate(range(min = 1, max = 65535))]
    pub internal_port: Option<u16>,
}

#[utoipa::path(get, path = "/api/v1/projects/{project_id}/services", security(("bearer_auth" = [])), params(("project_id" = Uuid, Path)), responses((status = 200, body = [AppService])))]
pub async fn list_services(
    _user: CurrentUser,
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
) -> Result<Json<Vec<AppService>>, ApiError> {
    Ok(Json(
        AppServiceService::new(state.db)
            .list_services(project_id)
            .await?,
    ))
}

#[utoipa::path(post, path = "/api/v1/projects/{project_id}/services", security(("bearer_auth" = [])), params(("project_id" = Uuid, Path)), request_body = CreateServiceRequest, responses((status = 200, body = AppService)))]
pub async fn create_service(
    _user: CurrentUser,
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
    Valid(Json(payload)): Valid<Json<CreateServiceRequest>>,
) -> Result<Json<AppService>, ApiError> {
    Ok(Json(
        AppServiceService::new(state.db)
            .create_service(CreateAppServiceInput {
                project_id,
                environment_id: payload.environment_id,
                name: payload.name,
                deploy_kind: payload.deploy_kind,
                image: payload.image,
                compose_file: payload.compose_file,
                dockerfile: payload.dockerfile,
                internal_port: payload.internal_port,
            })
            .await?,
    ))
}

#[utoipa::path(get, path = "/api/v1/services/{id}", security(("bearer_auth" = [])), params(("id" = Uuid, Path)), responses((status = 200, body = AppService)))]
pub async fn get_service(
    _user: CurrentUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<AppService>, ApiError> {
    Ok(Json(
        AppServiceService::new(state.db).get_service(id).await?,
    ))
}
