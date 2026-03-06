use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use crate::{
    AppState,
    http::request::inventory_request::GetListRequest,
    utils::{
        errors::{ AppResult},
        extractors::{AppJson, AppPath},
        response::{create_response},
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

    body.validate()?;

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
