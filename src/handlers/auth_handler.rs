use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use bcrypt::hash;
use serde_json::json;
use tower_http::body;

use crate::{
    AppState,
    models::auth_model::Account,
    models::request::auth_request::{LoginRequest, SignupRequest},
    utils::{
        errors::{AppError, AppResult},
        extractors::AppJson,
        response::create_response_with_data,
    },
};

pub async fn login(
    State(state): State<AppState>,
    AppJson(body): AppJson<LoginRequest>,
) -> AppResult<impl IntoResponse> {
    body.validate()?;

    let account = sqlx::query_as::<_, Account>(
        "
        SELECT player_id, username, password, email, age, 
               rank, gold, cash, experience, nickname, create_time 
        FROM accounts 
        WHERE username = $1",
    )
    .bind(&body.username)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::Unauthorized("Invalid username".into()))?;

    Ok((
        StatusCode::CREATED,
        Json(create_response_with_data(
            200,
            &"Login successful".to_string(),
            Some(json!(account)),
        )),
    ))
}

pub async fn sign_up(
    State(state): State<AppState>,
    AppJson(body): AppJson<SignupRequest>,
) -> AppResult<impl IntoResponse> {

    body.validate()?;

    let count_username =
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM accounts WHERE username = $1")
            .bind(&body.username)
            .fetch_one(&state.db)
            .await?;

    if count_username > 1 {
        return Err(AppError::Conflict(
            format!(
                "Username {} sudah terdaftar, silahkan menggunakan username yang lain !",
                &body.username
            )
            .into(),
        ));
    }

    let count_email =
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM accounts WHERE email = $1")
            .bind(&body.email)
            .fetch_one(&state.db)
            .await?;

    if count_email > 1 {
        return Err(AppError::Conflict(
            format!(
                "Email {} sudah terdaftar, silahkan menggunakan email yang lain !",
                &body.email
            )
            .into(),
        ));
    }

    let password_hashed =
        hash(&body.password, 15).map_err(|e| AppError::InternalError(e.to_string()));


    Ok((
        StatusCode::OK,
        Json(create_response_with_data(
            200,
            &"Signup successful".to_string(),
            Some(json!({"has": "password_hashed"})),
        )),
    ))
}
