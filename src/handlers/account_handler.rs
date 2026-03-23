// feature next on
// update password, email

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use bcrypt::{hash, verify};
use serde_json::json;
use std::collections::HashMap;

use crate::{
    AppState,
    http::{query_params::account_query_params::GetDataAccountQuery, request::account_request::{ChangeEmail, ChangePassword}},
    models::account_model::{Account, AccountChangeEmail, AccountChangePassword},
    utils::{
        courier, datetime,
        errors::{AppError, AppResult},
        extractors::{AppJson, AppQuery},
        response::{create_response, create_response_with_data},
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
    let result_verify_new_password_same = verify(&body.new_password, &account.password)
        .map_err(|e| AppError::InternalError(e.to_string()))?;

    // when result verify password is true = same password
    if result_verify_new_password_same == true {
        return Err(AppError::Unauthorized("Password cant be same".into()));
    }

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
                  password_text = $2,
                  update_time = $3
              WHERE player_id = $4",
    )
    //password hash
    .bind(&password_hashed)
    //password text
    .bind(&body.new_password)
    //update time
    .bind(&datetime::get_date_time_now())
    //player id
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

    // tracing::event!(tracing::Level::INFO, "Date time {}", &datetime::get_date_time_now());

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
            &"Berhasil mengganti email !".to_string(),
        )),
    ))
}

pub async fn get_account(
    State(state): State<AppState>,
    AppQuery(query): AppQuery<GetDataAccountQuery>,
) -> AppResult<impl IntoResponse> {
    // validation request
    query.validate()?;
    let player_id = query.player_id;

    // checking account
    let account = sqlx::query_as::<_, Account>(
        "
        SELECT player_id, username, password, password_text, email, age, 
               rank, gold, cash, experience, nickname, pc_cafe, access_level,
               create_time, update_time 
        FROM accounts 
        WHERE player_id = $1",
    )
    .bind(player_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Invalid Player Id".into()))?;

    Ok((
        StatusCode::OK,
        Json(create_response_with_data(
            200,
            &"Login successful".to_string(),
            Some(json!(account)),
        )),
    ))
}
