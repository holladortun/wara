use axum::{Json, Router, extract::State, routing::get};
use axum_valid::Valid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::{
    entities::users::{Role, User},
    errors::ApiError,
    services::auth::{AdminUser, AuthService, InviteUserInput},
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new().route("/admin/users", get(list_users).post(invite_user))
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct InviteUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1, max = 120))]
    pub name: String,
    pub role: Role,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct InviteUserResponse {
    pub user: User,
    pub invite_link: String,
    pub expires_at: DateTime<Utc>,
}

#[utoipa::path(get, path = "/api/v1/admin/users", security(("bearer_auth" = [])), responses((status = 200, body = [User])))]
pub async fn list_users(
    _admin: AdminUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, ApiError> {
    Ok(Json(
        AuthService::new(state.db, state.config)
            .list_users()
            .await?,
    ))
}

#[utoipa::path(post, path = "/api/v1/admin/users", security(("bearer_auth" = [])), request_body = InviteUserRequest, responses((status = 200, body = InviteUserResponse)))]
pub async fn invite_user(
    _admin: AdminUser,
    State(state): State<AppState>,
    Valid(Json(payload)): Valid<Json<InviteUserRequest>>,
) -> Result<Json<InviteUserResponse>, ApiError> {
    let output = AuthService::new(state.db, state.config)
        .invite_user(InviteUserInput {
            email: payload.email,
            name: payload.name,
            role: payload.role,
        })
        .await?;
    Ok(Json(InviteUserResponse {
        user: output.user,
        invite_link: output.invite_link,
        expires_at: output.expires_at,
    }))
}
