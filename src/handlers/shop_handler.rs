/// feature is for shop
/// top up cash, gold, tag
/// top up ribbon, ensign, medal, master_medal
/// buy weapon
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::{
    AppState,
    models::{
        inventory_model::{StateAccountMedal, StateAccountMoney},
        request::inventory_request::{TopUpMedalRequest, TopUpMoneyRequest},
    },
    utils::{
        errors::{AppError, AppResult},
        extractors::AppJson,
        response::{create_response, create_response_with_data},
    },
};

// feature next on
// top up cash, gold, tag
// top up ribbon, ensign, medal, master_medal

/// # Feature for top up cash, gold and tag
/// # URL : `{BASE_URL}/api/inventory/top-up-money`
pub async fn top_up_money(
    State(state): State<AppState>,
    AppJson(body): AppJson<TopUpMoneyRequest>,
) -> AppResult<impl IntoResponse> {
    body.validate()?;

    let mut tx = state.db.begin().await?;

    let current_account = sqlx::query_as::<_, StateAccountMoney>(
        "SELECT cash, gold, tag FROM account WHERE player_id = $1 FOR UPDATE",
    )
    .bind(body.player_id)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or_else(|| AppError::NotFound("current account not found".into()))?;

    let update_value = match body.top_up_type.as_str() {
        "cash" => body.value + current_account.cash,
        "gold" => body.value + current_account.gold,
        "tag" => body.value + current_account.tag,
        _ => return Err(AppError::BadRequest("type top up not found".into())),
    };

    let query_row = format!(
        "UPDATE accounts SET {} = $1 WHERE player_id = $2",
        body.top_up_type
    );

    // update password account from db
    sqlx::query(&query_row)
        // update value
        .bind(update_value)
        // player id
        .bind(body.player_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::DatabaseError(e))?;

    // db transaction commited
    tx.commit().await.map_err(|e| AppError::DatabaseError(e))?;

    // create message response
    Ok((
        StatusCode::OK,
        Json(create_response(
            200,
            &format!("Top up {} berhasil", body.top_up_type),
        )),
    ))
}

/// # Feature for top up medal
/// # URL : `{BASE_URL}/api/inventory/top-up-medal`
pub async fn top_up_medal(
    State(state): State<AppState>,
    AppJson(body): AppJson<TopUpMedalRequest>,
) -> AppResult<impl IntoResponse> {
    body.validate()?;

    let mut tx = state.db.begin().await?;

    // !property perlu di adjust
    let current_account = sqlx::query_as::<_, StateAccountMedal>(
        "SELECT cash, gold, tag FROM account WHERE player_id = $1 FOR UPDATE",
    )
    .bind(body.player_id)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or_else(|| AppError::NotFound("current account not found".into()))?;

    let update_value = match body.top_up_type.as_str() {
        "cash" => body.value + current_account.cash,
        "gold" => body.value + current_account.gold,
        "tag" => body.value + current_account.tag,
        _ => return Err(AppError::BadRequest("type top up not found".into())),
    };

    let query_row = format!(
        "UPDATE accounts SET {} = $1 WHERE player_id = $2",
        body.top_up_type
    );

    // update password account from db
    sqlx::query(&query_row)
        // update value
        .bind(update_value)
        // player id
        .bind(body.player_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::DatabaseError(e))?;

    // db transaction commited
    tx.commit().await.map_err(|e| AppError::DatabaseError(e))?;

    // create message response
    Ok((
        StatusCode::OK,
        Json(create_response(
            200,
            &format!("Top up {} berhasil", body.top_up_type),
        )),
    ))
}
