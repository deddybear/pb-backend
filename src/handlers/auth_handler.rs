use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use base64::{Engine, prelude::BASE64_STANDARD};
use bcrypt::{hash, verify};
use serde_json::json;

use crate::{
    AppState,
    models::{
        auth_model::{Account, AccountRecovery},
        request::auth_request::{AccountRecoveryRequest, LoginRequest, SignupRequest},
    },
    utils::{
        courier,
        errors::{AppError, AppResult},
        extractors::AppJson,
        jwt::generate_token,
        response::{create_response, create_response_with_data},
    },
};

/// # Fungsi untuk login akun
/// # URL : `{BASE_URL}/api/auth/login`
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
    )
    .unwrap();

    let token_base_encode64 = BASE64_STANDARD.encode(token);

    Ok((
        StatusCode::CREATED,
        Json(create_response_with_data(
            200,
            &"Login successful".to_string(),
            Some(json!({
                "token" : "Bearer ".to_string() + &token_base_encode64,
                "data_account": account
            })),
        )),
    ))
}

/// # Fungsi untuk pendaftaran akun baru
/// # URL : `{BASE_URL}/api/auth/signup`
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

/// # Fungsi diperuntukan untuk user yang lupa password
/// # URL : `{BASE_URL}/api/auth/account-recovery`
pub async fn account_recovery(
    State(state): State<AppState>,
    AppJson(body): AppJson<AccountRecoveryRequest>,
) -> AppResult<impl IntoResponse> {
    if state.config.smtp_enable == false {
        return Ok((
            StatusCode::OK,
            Json(create_response(200, &"fiture tidak dibuka".to_string())),
        ));
    }

    body.validate()?;

    // check email dari username tersebut
    let account = sqlx::query_as::<_, AccountRecovery>(
        "
        SELECT email, nickname
        FROM accounts 
        WHERE username = $1",
    )
    .bind(&body.username)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::Unauthorized("Invalid username".into()))?;

    // jika account tidak mempunyai email
    if account.email.is_empty() {
        return Err(AppError::InternalError(
            format!("Email dari akun ini kosong bro {}", &body.username).into(),
        ));
    }

    // mendapatkan template email
    // ! ini masih belum
    let template_email = tokio::fs::read_to_string("/src/template/password_recovery.html")
        .await
        .map_err(|e| AppError::InternalError(format!("Gagal membaca template email : {}", e)))?;

    // melakukan pengiriman email
    courier::send_mail(
        state.config.smtp_username.to_string(),
        state.config.smtp_password.to_string(),
        state.config.smtp_host.to_string(),
        state.config.smtp_port,
        "PB ITKI - Account Recovery".to_string(),
        state.config.smtp_address_from.to_string(),
        account.nickname,
        account.email,
        "Password Recovery".to_string(),
        template_email,
    )
    .map_err(|e| AppError::InternalError(format!("Gagal mengirimkan email: {}", e)))?;

    Ok((
        StatusCode::OK,
        Json(create_response(
            200,
            &"Password Baru anda telah dikirim melalui email yang terdaftar".to_string(),
        )),
    ))
}
