use axum::{Json, extract::State, http::StatusCode, response::IntoResponse, response::Response};

use serde_json::json;

use crate::{
    AppState,
    models::{
        auth_model::{LoginRequest, SignupRequest},
        response_model::TemplateWithData,
    },
    utils::response::{create_response, create_response_with_data},
};

pub async fn login(
    State(_state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> impl IntoResponse {
    return (
        StatusCode::OK,
        Json(json!({
            "code_http": 200,
            "message": "Login successful",
            "data": body
        })),
    )
        .into_response();
}

pub async fn sign_up(
    State(state): State<AppState>,
    Json(body): Json<SignupRequest>,
) -> impl IntoResponse {

    

    return (
        StatusCode::OK,
        Json(serde_json::json!(TemplateWithData {
            code_http: 200,
            message: "Signup successful".to_string(),
            data: Some(json!(body))
        })),
    )
        .into_response();
}
