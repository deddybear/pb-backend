use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::{
    AppState,
    http::request::inventory_request::GetListRequest,
    models::inventory_model::{StateAccountMedal, StateAccountMoney},
    utils::{
        errors::{AppError, AppResult},
        extractors::{AppJson, AppPath, AppQuery},
        response::{create_response, create_response_with_data},
    },
};

// feature next on
// read inventory
// delete item from inventory
// read detail item

pub async fn get_list(
    State(state): State<AppState>,
    AppJson(body): AppJson<GetListRequest>,
) -> AppResult<impl IntoResponse> {
    Ok((
        StatusCode::OK,
        Json(create_response(200, &format!("Feature terbuka"))),
    ))
}

pub async fn get_data(
    State(state): State<AppState>,
    AppPath(id): AppPath<u64>,
) -> AppResult<impl IntoResponse> {
    Ok((
        StatusCode::OK,
        Json(create_response(200, &format!("Feature terbuka"))),
    ))
}

pub async fn delete(
    State(state): State<AppState>,
    AppPath(id): AppPath<u64>,
) -> AppResult<impl IntoResponse> {
    Ok((
        StatusCode::OK,
        Json(create_response(200, &format!("Feature terbuka"))),
    ))
}
