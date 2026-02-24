use crate::models::response_model::Template;
use axum::{Json, http::StatusCode, response::IntoResponse};
use http::{method::Method, uri::Uri};

pub async fn handle_any_error(method: Method, uri: Uri, err: anyhow::Error) -> impl IntoResponse {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(Template {
            code_http: 500,
            message: "An unexpected error occurred : ".to_string()
                + &err.to_string()
                + " on "
                + &method.to_string()
                + " "
                + &uri.to_string(),
        }).into_response(),
    );
}

pub async fn handle_any_error_2(
    method: Method,
    uri: Uri,
    err: anyhow::Error,
) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("`{method} {uri}` failed with {err}"),
    )
}
