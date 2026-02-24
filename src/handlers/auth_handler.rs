use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use serde_json::json;

use crate::{
    AppState,
    models::auth_model::{Account, LoginRequest, SignupRequest},
    utils::{
        errors::{AppError, AppResult, extractors::AppJson},
        response::create_response_with_data,
    },
};

pub async fn login(
    State(state): State<AppState>,
    AppJson(body): AppJson<LoginRequest>,
) -> AppResult<impl IntoResponse> {
    body.validate()?;

    let account = sqlx::query_as::<_, Account>("SELECT * FROM accounts WHERE username = $1")
        .bind(&body.username)
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid username".into()))?;

    Ok((
        StatusCode::CREATED,
        Json(create_response_with_data(
            200,
            &"Login successful".to_string(),
            Some(json!(body)),
        )),
    ))
}

pub async fn sign_up(
    State(state): State<AppState>,
    Json(body): Json<SignupRequest>,
) -> AppResult<impl IntoResponse> {
    Ok((
        StatusCode::CREATED,
        Json(create_response_with_data(
            200,
            &"Signup successful".to_string(),
            Some(json!(body)),
        )),
    ))
}
