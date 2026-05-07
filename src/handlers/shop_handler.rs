use crate::{
    AppState,
    http::{
        query_params::shop_query_params::ListShopWeaponQuery,
        request::shop_request::{TopUpMedalRequest, TopUpMoneyRequest},
    },
    models::{
        inventory_model::{StateAccountMedal, StateAccountMoney},
        shop_model::ShopWeapon,
    },
    utils::{
        errors::{AppError, AppResult},
        extractors::{AppJson, AppQuery},
        response::{create_response, create_response_with_data},
    },
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

/// feature is for shop
/// top up cash, gold, tags
/// top up ribbon, ensign, medal, master_medal
/// buy weapon, chara, head

/// # Feature for top up cash, gold and tags
/// # URL : `{BASE_URL}/api/inventory/top-up-money`
pub async fn top_up_money(
    State(state): State<AppState>,
    AppJson(body): AppJson<TopUpMoneyRequest>,
) -> AppResult<impl IntoResponse> {
    body.validate()?;

    let mut tx = state.db.begin().await?;

    let current_account = sqlx::query_as::<_, StateAccountMoney>(
        "SELECT cash, gold, tags, online FROM accounts WHERE player_id = $1 FOR UPDATE",
    )
    .bind(body.player_id)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or_else(|| AppError::NotFound("current account not found".into()))?;

    if current_account.online == true {
        return Err(AppError::Forbidden(
            "akun anda berstatus online mohon untuk logout terlebih dahulu".into(),
        ));
    }

    let update_value = match body.top_up_type.as_str() {
        "cash" => body.value + current_account.cash,
        "gold" => body.value + current_account.gold,
        "tags" => body.value + current_account.tags,
        _ => return Err(AppError::BadRequest("type top up not found".into())),
    };

    // check batasan value setiap top up
    // point 10jt, cash 2jt, tag 1jt
    let (max_value, label) = match body.top_up_type.as_str() {
        "cash" => (2_000_000, "cash (maks. 2jt)"),
        "gold" => (10_000_000, "point (maks. 10jt)"),
        "tags" => (1_000_000, "tags (maks. 1jt)"),
        _ => return Err(AppError::BadRequest("type top up not found".into())),
    };

    if update_value > max_value {
        return Err(AppError::BadRequest(
            format!(
                "gagal melakukan top up {} karena melebihi batas maksimal {}",
                body.top_up_type, label
            )
            .into(),
        ));
    }

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
        "SELECT ribbon, ensign, medal, master_medal, online FROM accounts WHERE player_id = $1 FOR UPDATE",
    )
    .bind(body.player_id)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or_else(|| AppError::NotFound("current account not found".into()))?;

    if current_account.online == true {
        return Err(AppError::Forbidden(
            "akun anda berstatus online mohon untuk logout terlebih dahulu".into(),
        ));
    }

    let update_value = match body.top_up_type.as_str() {
        "ribbon" => body.value + current_account.ribbon,
        "ensign" => body.value + current_account.ensign,
        "medal" => body.value + current_account.medal,
        "master_medal" => body.value + current_account.master_medal,
        _ => return Err(AppError::BadRequest("type top up not found".into())),
    };

    // check batasan value setiap top up
    // ribbon 200, ensign 150, medal 100, master_medal 50
    let (max_value, label) = match body.top_up_type.as_str() {
        "ribbon" => (200, "cash (maks. 200)"),
        "ensign" => (150, "point (maks. 150)"),
        "medal" => (100, "tags (maks. 100)"),
        "master_medal" => (50, "tags (maks. 50)"),
        _ => return Err(AppError::BadRequest("type top up not found".into())),
    };

    if update_value > max_value {
        return Err(AppError::BadRequest(
            format!(
                "gagal melakukan top up {} karena melebihi batas maksimal {}",
                body.top_up_type, label
            )
            .into(),
        ));
    }

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
        SELECT * FROM view_primary_weapon_shop 
        WHERE item_name ilike $1
        AND item_visible = true
        ORDER BY item_name ASC LIMIT $2 OFFSET $3",
        )
        .bind(pattern)
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await?
    } else {
        sqlx::query_as::<_, ShopWeapon>(
            "
        SELECT * FROM view_primary_weapon_shop
        WHERE item_visible = true
        ORDER BY item_name ASC LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await?
    };

    Ok((
        StatusCode::OK,
        Json(create_response_with_data(
            200,
            "success",
            serde_json::json!({
                "meta": {"page": page, "limit": limit},
                "data": list_data_weapon
            })
            .into(),
        )),
    ))
}

/// # Feature for get list weapon secondary
/// # URL : `{BASE_URL}/api/inventory/list_shop_weapon_secondary`
pub async fn list_shop_weapon_secondary(
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
        SELECT * FROM view_secondary_weapon_shop 
        WHERE item_name ilike $1
        AND item_visible = true
        ORDER BY item_name ASC LIMIT $2 OFFSET $3",
        )
        .bind(&pattern)
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await?
    } else {
        sqlx::query_as::<_, ShopWeapon>(
            "
        SELECT * FROM view_secondary_weapon_shop
        WHERE item_visible = true
        ORDER BY item_name ASC LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await?
    };

    Ok((
        StatusCode::OK,
        Json(create_response_with_data(
            200,
            "success",
            serde_json::json!({
                "meta": {"page": page, "limit": limit},
                "data": list_data_weapon
            })
            .into(),
        )),
    ))
}

/// # Feature for get list weapon melee
/// # URL : `{BASE_URL}/api/inventory/list_shop_weapon_melee`
pub async fn list_shop_weapon_melee(
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
        SELECT * FROM view_melee_weapon_shop 
        WHERE item_name ilike $1
        AND item_visible = true
        ORDER BY item_name ASC LIMIT $2 OFFSET $3",
        )
        .bind(&pattern)
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await?
    } else {
        sqlx::query_as::<_, ShopWeapon>(
            "
        SELECT * FROM view_melee_weapon_shop
        WHERE item_visible = true
        ORDER BY item_name ASC LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await?
    };

    Ok((
        StatusCode::OK,
        Json(create_response_with_data(
            200,
            "success",
            serde_json::json!({
                "meta": {"page": page, "limit": limit},
                "data": list_data_weapon
            })
            .into(),
        )),
    ))
}

/// # Feature for get list weapon explosive
/// # URL : `{BASE_URL}/api/inventory/list_shop_weapon_explosive`
pub async fn list_shop_weapon_explosive(
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
        SELECT * FROM view_explosive_weapon_shop 
        WHERE item_name ilike $1
        AND item_visible = true
        ORDER BY item_name ASC LIMIT $2 OFFSET $3",
        )
        .bind(&pattern)
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await?
    } else {
        sqlx::query_as::<_, ShopWeapon>(
            "
        SELECT * FROM view_explosive_weapon_shop
        WHERE item_visible = true
        ORDER BY item_name ASC LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await?
    };

    Ok((
        StatusCode::OK,
        Json(create_response_with_data(
            200,
            "success",
            serde_json::json!({
                "meta": {"page": page, "limit": limit},
                "data": list_data_weapon
            })
            .into(),
        )),
    ))
}

/// # Feature for get list character shop
/// # URL : `{BASE_URL}/api/inventory/list_shop_character`
pub async fn list_shop_character(
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
        SELECT * FROM view_character_shop 
        WHERE item_name ilike $1
        AND item_visible = true
        ORDER BY item_name ASC LIMIT $2 OFFSET $3",
        )
        .bind(&pattern)
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await?
    } else {
        sqlx::query_as::<_, ShopWeapon>(
            "
        SELECT * FROM view_character_shop
        WHERE item_visible = true
        ORDER BY item_name ASC LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await?
    };

    Ok((
        StatusCode::OK,
        Json(create_response_with_data(
            200,
            "success",
            serde_json::json!({
                "meta": {"page": page, "limit": limit},
                "data": list_data_weapon
            })
            .into(),
        )),
    ))
}

/// # Feature for get list character shop
/// # URL : `{BASE_URL}/api/inventory/list_shop_headgear`
pub async fn list_shop_headgear(
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
        SELECT * FROM view_headgear_shop 
        WHERE item_name ilike $1
        AND item_visible = true
        ORDER BY item_name ASC LIMIT $2 OFFSET $3",
        )
        .bind(&pattern)
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await?
    } else {
        sqlx::query_as::<_, ShopWeapon>(
            "
        SELECT * FROM view_headgear_shop
        WHERE item_visible = true
        ORDER BY item_name ASC LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await?
    };

    Ok((
        StatusCode::OK,
        Json(create_response_with_data(
            200,
            "success",
            serde_json::json!({
                "meta": {"page": page, "limit": limit},
                "data": list_data_weapon
            })
            .into(),
        )),
    ))
}

/// # Feature for get list character shop
/// # URL : `{BASE_URL}/api/inventory/list_shop_consume`
pub async fn list_shop_consume(
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
        SELECT * FROM view_cosume_shop 
        WHERE item_name ilike $1
        AND item_visible = true
        ORDER BY item_name ASC LIMIT $2 OFFSET $3",
        )
        .bind(&pattern)
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await?
    } else {
        sqlx::query_as::<_, ShopWeapon>(
            "
        SELECT * FROM view_cosume_shop
        WHERE item_visible = true
        ORDER BY item_name ASC LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await?
    };

    Ok((
        StatusCode::OK,
        Json(create_response_with_data(
            200,
            "success",
            serde_json::json!({
                "meta": {"page": page, "limit": limit},
                "data": list_data_weapon
            })
            .into(),
        )),
    ))
}

// pub async fn buy() {}
