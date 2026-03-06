// feature next on
// update password, email

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use bcrypt::{hash, verify};
use std::collections::HashMap;

use crate::{
    AppState,
    http::request::account_request::{ChangeEmail, ChangePassword},
    models::account_model::{AccountChangeEmail, AccountChangePassword},
    utils::{
        courier, datetime,
        errors::{AppError, AppResult},
        extractors::AppJson,
        response::{create_response},
    },
};

/// # Feature for change password
/// # URL : `{BASE_URL}/api/account/change-password`
pub async fn change_password(
    State(state): State<AppState>,
    AppJson(body): AppJson<ChangePassword>,
) -> AppResult<impl IntoResponse> {
    //validate request
    body.validate()?;

    let account = sqlx::query_as::<_, AccountChangePassword>(
        "SELECT password, nickname, email
        FROM accounts 
        WHERE player_id = $1",
    )
    .bind(&body.player_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::Unauthorized("Invalid player_id".into()))?;

    // verify password from input client with password in database
    let result_verify_password = verify(&body.old_password, &account.password)
        .map_err(|e| AppError::InternalError(e.to_string()))?;

    // when result verify password is false
    if result_verify_password == false {
        return Err(AppError::Unauthorized("Invalid password".into()));
    }

    // hash password
    let password_hashed =
        hash(&body.new_password, 15).map_err(|e| AppError::InternalError(e.to_string()))?;

    // database transaction
    let mut tx = state.db.begin().await?;

    // update password account from db
    sqlx::query(
        "UPDATE accounts 
              SET password = $1,
                  update_time = $2
              WHERE player_id = $3",
    )
    //password
    .bind(&password_hashed)
    .bind(&datetime::get_date_time_now())
    .bind(&body.player_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| AppError::DatabaseError(e))?;

    // db transaction commited
    tx.commit().await.map_err(|e| AppError::DatabaseError(e))?;

    // if smtp not enable then no need send email
    if state.config.smtp_enable == false {
        return Ok((
            StatusCode::OK,
            Json(create_response(
                200,
                &"Berhasil mengganti Password !".to_string(),
            )),
        ));
    }

    // get template email html
    let template_email = tokio::fs::read_to_string("./src/template/password_change.html")
        .await
        .map_err(|e| AppError::InternalError(format!("Gagal membaca template email : {}", e)))?;

    // set variable for email
    let mut variables: HashMap<&str, &str> = HashMap::new();
    let year_now = datetime::get_year_now().to_string();
    let email_support = &state.config.smtp_address_from.to_string();

    variables.insert("nickname", &account.nickname);
    variables.insert("email_user", &account.email);
    variables.insert("email_support", &email_support);
    variables.insert("year_now", &year_now);

    // insign variable
    let email_content = courier::render_template(&template_email, &variables);

    // send email
    courier::send_mail(
        state,
        "PB ITKI - Password Change".to_string(),
        account.nickname,
        account.email,
        "Password Change".to_string(),
        email_content,
    )
    .map_err(|e| AppError::InternalError(format!("Gagal mengirimkan email: {}", e)))?;

    Ok((
        StatusCode::OK,
        Json(create_response(
            200,
            &"Berhasil mengganti Password !".to_string(),
        )),
    ))
}

/// # Feature for change email
/// # URL : `{BASE_URL}/api/account/change-email`
pub async fn change_email(
    State(state): State<AppState>,
    AppJson(body): AppJson<ChangeEmail>,
) -> AppResult<impl IntoResponse> {
    //validate request
    body.validate()?;

    let account = sqlx::query_as::<_, AccountChangeEmail>(
        "SELECT email, nickname
        FROM accounts 
        WHERE player_id = $1",
    )
    .bind(&body.player_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::Unauthorized("Invalid player_id".into()))?;

    // verify email from input client with email in database
    let result_verify_email = &body.old_email == &account.email;

    // when result verify password is false
    if result_verify_email == false {
        return Err(AppError::Unauthorized(
            "Invalid Email old not same in database".into(),
        ));
    }

    tracing::event!(tracing::Level::INFO, "Date time {}", &datetime::get_date_time_now());
    

    // database transaction
    let mut tx = state.db.begin().await?;

    // update password account from db
    sqlx::query(
        "UPDATE accounts 
              SET email = $1,
                  update_time = $2
              WHERE player_id = $3",
    )
    //password
    .bind(&body.new_email)
    .bind(&datetime::get_date_time_now())
    .bind(&body.player_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| AppError::DatabaseError(e))?;

    // db transaction commited
    tx.commit().await.map_err(|e| AppError::DatabaseError(e))?;

    // if smtp not enable then no need send email
    if state.config.smtp_enable == false {
        return Ok((
            StatusCode::OK,
            Json(create_response(
                200,
                &"Berhasil mengganti Email !".to_string(),
            )),
        ));
    }

    // get template email html
    let template_email = tokio::fs::read_to_string("./src/template/email_change.html")
        .await
        .map_err(|e| AppError::InternalError(format!("Gagal membaca template email : {}", e)))?;

    // set variable for email
    let mut variables: HashMap<&str, &str> = HashMap::new();
    let year_now = datetime::get_year_now().to_string();
    let email_support = &state.config.smtp_address_from.to_string();

    variables.insert("nickname", &account.nickname);
    variables.insert("old_email", &body.old_email);
    variables.insert("new_email", &body.new_email);
    variables.insert("email_user", &account.email);
    variables.insert("email_support", &email_support);
    variables.insert("year_now", &year_now);

    // insign variable
    let email_content = courier::render_template(&template_email, &variables);

    // send email
    courier::send_mail(
        state,
        "PB ITKI - Email Change".to_string(),
        account.nickname,
        account.email,
        "Email Change".to_string(),
        email_content,
    )
    .map_err(|e| AppError::InternalError(format!("Gagal mengirimkan email: {}", e)))?;

    Ok((
        StatusCode::OK,
        Json(create_response(
            200,
            &"Berhasil mengganti Password !".to_string(),
        )),
    ))
}
