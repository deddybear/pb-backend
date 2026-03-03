use axum::{
    http::{Method, StatusCode, Uri},
    response::IntoResponse,
    Json,
};

use crate::http::response::general_response::Template;


/// Handles 404 Not Found errors for undefined routes.
///
/// This handler is invoked when a request is made to a route that does not exist.
/// It returns a JSON response with HTTP status code 404 and a descriptive error message
/// indicating the requested path.
///
/// # Arguments
///
/// * `uri` - The URI of the request that triggered the 404 error
///
/// # Returns
///
/// A tuple containing:
/// - `StatusCode::NOT_FOUND` - HTTP 404 status code
/// - A JSON response with error details

pub async fn handler_404(uri: Uri) -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(Template {
            code_http: StatusCode::NOT_FOUND.as_u16(),
            message: format!("Route '{}' not found", uri.path()),
        }),
    )
}

/// Handles 405 Not Found errors for undefined routes.
///
/// This handler is invoked when a request is made to a route that does not exist.
/// It returns a JSON response with HTTP status code 405 and a descriptive error message
/// indicating the requested path.
///
/// # Arguments
///
/// * `uri` - The URI of the request that triggered the 405 error
///
/// # Returns
///
/// A tuple containing:
/// - `StatusCode::METHOD_NOT_ALLOWED` - HTTP 405 status code
/// - A JSON response with error details
pub async fn handler_405(method: Method, uri: Uri) -> impl IntoResponse {
    (
        StatusCode::METHOD_NOT_ALLOWED,
        Json(Template {
            code_http: StatusCode::METHOD_NOT_ALLOWED.as_u16(),
            message: format!("Method '{}' not allowed for route '{}'", method, uri.path()),
        }),
    )
}
