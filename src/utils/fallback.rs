use axum::{
    http::{Method, StatusCode, Uri},
    response::IntoResponse,
    Json,
};

use crate::models::response_model::Template;

/// Dipanggil saat route tidak ditemukan sama sekali → 404
pub async fn handler_404(uri: Uri) -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(Template {
            code_http: StatusCode::NOT_FOUND.as_u16(),
            message: format!("Route '{}' not found", uri.path()),
        }),
    )
}

/// Dipanggil saat route ada tapi method tidak cocok → 405
pub async fn handler_405(method: Method, uri: Uri) -> impl IntoResponse {
    (
        StatusCode::METHOD_NOT_ALLOWED,
        Json(Template {
            code_http: StatusCode::METHOD_NOT_ALLOWED.as_u16(),
            message: format!("Method '{}' not allowed for route '{}'", method, uri.path()),
        }),
    )
}
