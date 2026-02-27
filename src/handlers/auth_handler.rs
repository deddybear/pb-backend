use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use bcrypt::{hash, verify};
use serde_json::json;

use crate::{
    AppState,
    models::{
        auth_model::Account,
        request::auth_request::{LoginRequest, SignupRequest},
    },
    utils::{
        errors::{AppError, AppResult},
        extractors::AppJson,
        jwt::generate_token,
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
               rank, gold, cash, experience, nickname, pc_cafe, access_level,
               create_time, update_time 
        FROM accounts 
        WHERE username = $1",
    )
    .bind(&body.username)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::Unauthorized("Invalid username".into()))?;

    let result_verify_password = verify(&body.password, &account.password)
        .map_err(|e| AppError::InternalError(e.to_string()))?;

    if result_verify_password == false {
        return Err(AppError::Unauthorized("Invalid password".into()));
    }

    let token = generate_token(
        &account.player_id,
        &account.email,
        &account.access_level,
        &state.config.jwt_secret,
    ).unwrap();

    Ok((
        StatusCode::CREATED,
        Json(create_response_with_data(
            200,
            &"Login successful".to_string(),
            Some(json!({
                "token" : token,
                "data_account": account
            })),
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

    if count_username > 0 {
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

    if count_email > 0 {
        return Err(AppError::Conflict(
            format!(
                "Email {} sudah terdaftar, silahkan menggunakan email yang lain !",
                &body.email
            )
            .into(),
        ));
    }

    let password_hashed =
        hash(&body.password, 15).map_err(|e| AppError::InternalError(e.to_string()))?;

    let mut tx = state.db.begin().await?;

    sqlx::query(
        "INSERT INTO accounts (username, password, email, age, rank, experience, cash, gold, tags, pc_cafe) 
              VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
    )
    .bind(&body.username)
    .bind(&password_hashed)
    .bind(&body.email)
    .bind(&body.age)
    //rank
    .bind(31)
    .bind(1)
    //cash
    .bind(100000)
    //gold
    .bind(1000000)
    //tags
    .bind(100000)
    //pc_cafe 
    .bind(2)
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        // Jika email race condition (duplicate key), berikan pesan yang jelas
        if e.to_string().contains("duplicate key") {
            AppError::Conflict("Email already registered".into())
        } else {
            AppError::DatabaseError(e)
        }
    })?;

    tx.commit().await?;

    Ok((
        StatusCode::OK,
        Json(create_response_with_data(
            200,
            &"Signup successful".to_string(),
            Some(json!({"password_hash": password_hashed})),
        )),
    ))
}
