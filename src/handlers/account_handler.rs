// feature next on
// update password, email

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::{
    AppState,
    http::request::account_request::{ChangeEmail, ChangePassword},
    utils::{
        errors::{AppError, AppResult},
        extractors::AppJson,
        response::{create_response, create_response_with_data},
    },
};

/// # Feature for change password
/// # URL : `{BASE_URL}/api/account/change-password`
pub async fn change_password(
    State(state): State<AppState>,
    AppJson(body): AppJson<ChangePassword>,
) -> AppResult<impl IntoResponse> {
    body.validate()?;
    Ok((
        StatusCode::OK,
        Json(create_response(200, &"Fitur Update password".to_string())),
    ))
}

/// # Feature for change email
/// # URL : `{BASE_URL}/api/account/change-email`
pub async fn change_email(
    State(state): State<AppState>,
    AppJson(body): AppJson<ChangeEmail>,
) -> AppResult<impl IntoResponse> {
    body.validate()?;
    Ok((
        StatusCode::OK,
        Json(create_response(200, &"Fitur update email".to_string())),
    ))
}
