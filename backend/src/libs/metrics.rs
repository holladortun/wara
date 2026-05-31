use axum::response::IntoResponse;

pub async fn metrics_handler() -> impl IntoResponse {
    "# Wara platform metrics\n# Telemetry is opt-in and platform-only.\n"
}
