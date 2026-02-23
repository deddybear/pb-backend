mod auth;

use axum::{routing::get, Router};

use crate::AppState;

async fn home() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "code_http": 200,
        "status": "ok",
        "message": "hello :)"
    }))
}

async fn health_check() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "code_http": 200,
        "status": "ok",
        "message": "Server is running"
    }))
}

pub fn init_routes(state: AppState) -> Router {
    
    Router::new()
        .route("/", get(home))
        .route("/health", get(health_check))
        .nest("/api/auth", auth::auth_routes())
        .with_state(state)
}