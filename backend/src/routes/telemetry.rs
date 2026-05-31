use axum::{Json, Router, extract::State, routing::get};
use axum_valid::Valid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::{services::auth::AdminUser, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new().route(
        "/telemetry/settings",
        get(get_settings).put(update_settings),
    )
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct TelemetrySettings {
    pub enabled: bool,
    #[validate(length(min = 1, max = 120))]
    pub service_name: String,
    #[validate(url)]
    pub otlp_endpoint: Option<String>,
    pub platform_only: bool,
}

#[utoipa::path(get, path = "/api/v1/telemetry/settings", security(("bearer_auth" = [])), responses((status = 200, body = TelemetrySettings)))]
pub async fn get_settings(
    _admin: AdminUser,
    State(state): State<AppState>,
) -> Json<TelemetrySettings> {
    Json(TelemetrySettings {
        enabled: state.config.telemetry_enabled,
        service_name: state.config.otel_service_name,
        otlp_endpoint: state.config.otel_exporter_otlp_endpoint,
        platform_only: true,
    })
}

#[utoipa::path(put, path = "/api/v1/telemetry/settings", security(("bearer_auth" = [])), request_body = TelemetrySettings, responses((status = 200, body = TelemetrySettings)))]
pub async fn update_settings(
    _admin: AdminUser,
    Valid(Json(payload)): Valid<Json<TelemetrySettings>>,
) -> Json<TelemetrySettings> {
    Json(TelemetrySettings {
        platform_only: true,
        ..payload
    })
}
