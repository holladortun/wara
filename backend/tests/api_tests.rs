use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use wara_backend::{
    libs::{config::Config, db::Database},
    routes,
    state::AppState,
};

fn app() -> axum::Router {
    let state = AppState::new(Config::from_env(), Database::unavailable_for_tests());
    routes::router(state)
}

#[tokio::test]
async fn health_is_public() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn protected_routes_require_auth() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/api/v1/projects")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn openapi_is_exposed() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/api/openapi.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn invalid_json_payloads_are_rejected_by_validation() {
    let response = app()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"email":"not-an-email","password":""}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
