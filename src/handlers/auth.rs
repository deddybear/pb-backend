use crate::utils::response::create_response;
use axum::response::{IntoResponse, Response};
use http::StatusCode;

pub async fn login() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&create_response(200, "Login successful")).unwrap())
        .unwrap()
        .into_response()
}

pub async fn sign_up() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&create_response(200, "Sign up successful")).unwrap())
        .unwrap()
        .into_response()
}
