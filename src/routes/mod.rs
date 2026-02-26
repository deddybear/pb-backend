mod auth_route;
mod inventory_route;

use axum::{routing::get, Router};
use crate::{AppState, utils::fallback::{handler_404, handler_405}};

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
        .nest("/api/auth", auth_route::router(state.clone()))
        .nest("/api/inventory", inventory_route::router(state.clone()))
        .fallback(handler_404)
        .method_not_allowed_fallback(handler_405)
        .with_state(state)
}