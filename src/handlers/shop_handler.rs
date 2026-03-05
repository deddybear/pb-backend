use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use crate::{
    AppState,
    http::{query_params::shop_query_params::ListShopWeaponQuery, request::shop_request::{BuyWeaponRequest, TopUpMedalRequest, TopUpMoneyRequest}},
    models::{inventory_model::{StateAccountMedal, StateAccountMoney}, shop_model::ShopWeapon},
    utils::{
        errors::{AppError, AppResult},
        extractors::{AppJson, AppQuery},
        response::{create_response, create_response_with_data},
    },
};

/// feature is for shop
/// top up cash, gold, tag
/// top up ribbon, ensign, medal, master_medal
/// buy weapon, chara, head

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

/// # Feature for get list shop weapon primary
/// # URL : `{BASE_URL}/api/inventory/list-weapon-primary`
pub async fn list_shop_weapon_primary(
    State(state): State<AppState>,
    AppQuery(query): AppQuery<ListShopWeaponQuery>,
) -> AppResult<impl IntoResponse> {
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(10).clamp(1, 100);
    let offset = (page - 1) * limit;

    let list_data_weapon = if let Some(ref search) = query.search {
        let pattern = format!("%{}%", search);
        sqlx::query_as::<_, ShopWeapon>(
            "
        SELECT * FROM view_primary_weapon_shop WHERE item_name ilike $1
        ORDER BY item_nam ASC LIMIT $2 OFFSET $3",
        )
        .bind(&pattern)
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await?
    } else {
        sqlx::query_as::<_, ShopWeapon>(
            "
        SELECT * FROM view_primary_weapon_shop
        ORDER BY item_nam ASC LIMIT $2 OFFSET $3",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await?
    };

    Ok((
        StatusCode::OK,
        Json(
            create_response_with_data(
                200, 
                "success",
                serde_json::json!({
                    "meta": {"page": page, "limit": limit},
                    "data": list_data_weapon
                }).into()
            )
        ),
    ))
}

/// # Feature for buy weapon
/// # URL : `{BASE_URL}/api/inventory/buy-weapon`
pub async fn buy_weapon(
    State(state): State<AppState>,
    AppJson(body): AppJson<BuyWeaponRequest>,
) -> AppResult<impl IntoResponse> {
    body.validate()?;

    Ok((
        StatusCode::OK,
        Json(create_response(200, &format!("Berhasil Membeli Item"))),
    ))
}

pub async fn buy_chara() {}

pub async fn buy_head() {}
