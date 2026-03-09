use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use base64::{Engine, prelude::BASE64_STANDARD};
use std::collections::HashMap;
use bcrypt::{hash, verify};
use serde_json::json;

use crate::{
    AppState,
    http::request::auth_request::{AccountRecoveryRequest, LoginRequest, SignupRequest},
    models::account_model::{Account, AccountPasswordReset},
    utils::{
        datetime,
        courier,
        errors::{AppError, AppResult},
        extractors::AppJson,
        jwt::generate_token,
        rand,
        response::{create_response, create_response_with_data},
    },
};

/// # Feature for login account
/// # URL : `{BASE_URL}/api/auth/login`
pub async fn login(
    State(state): State<AppState>,
    AppJson(body): AppJson<LoginRequest>,
) -> AppResult<impl IntoResponse> {
    // validation request
    body.validate()?;

    // checking account
    let mut account = sqlx::query_as::<_, Account>(
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

    // verify password from input client with password in database
    let result_verify_password = verify(&body.password, &account.password)
        .map_err(|e| AppError::InternalError(e.to_string()))?;

    // when result verify password is false
    if result_verify_password == false {
        return Err(AppError::Unauthorized("Invalid password".into()));
    }

    // generate token
    let token = generate_token(
        &account.player_id,
        &account.email,
        &account.access_level,
        &state.config.jwt_secret,
    )
    .unwrap();

    // base encode
    let token_base_encode64 = BASE64_STANDARD.encode(token);

    account.password = "-".to_string();

    Ok((
        StatusCode::OK,
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

/// # Feature for login account
/// # URL : `{BASE_URL}/api/auth/login-app`
pub async fn login_app(
    State(state): State<AppState>,
    AppJson(body): AppJson<LoginRequest>,
) -> AppResult<impl IntoResponse> {
    // validation request
    body.validate()?;

    // checking account
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

    // verify password from input client with password in database
    let result_verify_password = verify(&body.password, &account.password)
        .map_err(|e| AppError::InternalError(e.to_string()))?;

    // when result verify password is false
    if result_verify_password == false {
        return Err(AppError::Unauthorized("Invalid password".into()));
    }

    Ok((
        StatusCode::OK,
        Json(create_response(
            200,
            &"Login successful".to_string()
        )),
    ))
}


/// # feature for register new account
/// # URL : `{BASE_URL}/api/auth/signup`
pub async fn sign_up(
    State(state): State<AppState>,
    AppJson(body): AppJson<SignupRequest>,
) -> AppResult<impl IntoResponse> {
    // validation req
    body.validate()?;

    // check username already used or not
    let count_username =
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM accounts WHERE username = $1")
            .bind(&body.username)
            .fetch_one(&state.db)
            .await?;

    // if username already used
    if count_username > 0 {
        return Err(AppError::Conflict(
            format!(
                "Username {} sudah terdaftar, silahkan menggunakan username yang lain !",
                &body.username
            )
            .into(),
        ));
    }

    // check email already used or not
    let count_email =
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM accounts WHERE email = $1")
            .bind(&body.email)
            .fetch_one(&state.db)
            .await?;

    // if email already used
    if count_email > 0 {
        return Err(AppError::Conflict(
            format!(
                "Email {} sudah terdaftar, silahkan menggunakan email yang lain !",
                &body.email
            )
            .into(),
        ));
    }

    // hashing passowrd
    let password_hashed =
        hash(&body.password, 15).map_err(|e| AppError::InternalError(e.to_string()))?;

    // database transaction
    let mut tx = state.db.begin().await?;

    // insert new account in database
    sqlx::query(
        "INSERT INTO accounts (username, password, email, age, rank, experience, cash, gold, tags, pc_cafe) 
              VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
    )
    //username
    .bind(&body.username)
    //password
    .bind(&password_hashed)
    //email
    .bind(&body.email)
    //age
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

    // db transaction commited
    tx.commit().await.map_err(|e| AppError::DatabaseError(e))?;

    Ok((
        StatusCode::CREATED,
        Json(create_response(
            201,
            &"Signup successful".to_string()
            // Some(json!({"password_hash": password_hashed})),
        )),
    ))
}

/// # feature for forgot password
/// # URL : `{BASE_URL}/api/auth/password-reset`
pub async fn password_reset(
    State(state): State<AppState>,
    AppJson(body): AppJson<AccountRecoveryRequest>,
) -> AppResult<impl IntoResponse> {
    // check env feature smtp is enable or not
    if state.config.smtp_enable == false {
        return Err(AppError::Forbidden("feature not opened yet !".to_string()));
    }

    // validation req
    body.validate()?;

    // check email from the username when not found will return response error
    let account = sqlx::query_as::<_, AccountPasswordReset>(
        "
        SELECT player_id, email, nickname
        FROM accounts 
        WHERE username = $1",
    )
    .bind(&body.username)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::Unauthorized("Invalid username".into()))?;

    // when account doesn't have email
    if account.email.is_empty() {
        return Err(AppError::InternalError(
            format!(
                "Email for this username are empty ! please call the administrator {}",
                &body.username
            )
            .into(),
        ));
    }

    // create new password randomize
    let new_password = rand::random_string(16, true, true, true).unwrap();

    // hashing password
    let password_hashed =
        hash(&new_password, 15).map_err(|e| AppError::InternalError(e.to_string()))?;

    // database transaction
    let mut tx = state.db.begin().await?;

    // update password account from db
    sqlx::query(
        "UPDATE accounts 
              SET password = $1,
                  update_time = NOW()
              WHERE player_id = $2",
    )
    //password
    .bind(&password_hashed)
    .bind(account.player_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| AppError::DatabaseError(e))?;

    // db transaction commited
    tx.commit().await.map_err(|e| AppError::DatabaseError(e))?;

    // encode new password
    let new_password_encode64 = BASE64_STANDARD.encode(new_password);

    // get template email html
    let template_email = tokio::fs::read_to_string("./src/template/password_reset.html")
        .await
        .map_err(|e| AppError::InternalError(format!("Gagal membaca template email : {}", e)))?;

    // set variable for email
    let mut variables: HashMap<&str, &str> = HashMap::new();
    let year_now = datetime::get_year_now().to_string();
    let email_support = &state.config.smtp_address_from.to_string();


    variables.insert("nickname", &account.nickname);
    variables.insert("new_password", &new_password_encode64);
    variables.insert("email_user", &account.email);
    variables.insert("email_support", &email_support);
    variables.insert("year_now", &year_now);

    // insign variable
    let email_content = courier::render_template(&template_email, &variables);

    // send email
    courier::send_mail(
        state,
        "PB ITKI - Password Reset".to_string(),
        account.nickname,
        account.email,
        "Password Reset".to_string(),
        email_content,
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
