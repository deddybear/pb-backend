pub mod extractors;
pub mod fallback;

use axum::{
    extract::rejection::{JsonRejection, PathRejection, QueryRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use serde_json::json;
use thiserror::Error;
use validator::ValidationErrors;

use crate::models::response_model::Template;


#[derive(Error, Debug)]
pub enum AppError {
    // ── 400 Bad Request ──────────────────────────────────────────────────────
    #[error("Bad request: {0}")]
    BadRequest(String),

    // ── 401 Unauthorized ─────────────────────────────────────────────────────
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    // ── 403 Forbidden ────────────────────────────────────────────────────────
    #[error("Forbidden: {0}")]
    Forbidden(String),

    // ── 404 Not Found ────────────────────────────────────────────────────────
    #[error("{0} not found")]
    NotFound(String),

    // ── 409 Conflict ─────────────────────────────────────────────────────────
    #[error("Conflict: {0}")]
    Conflict(String),

    // ── 413 Payload Too Large ────────────────────────────────────────────────
    #[error("Payload too large: {0}")]
    PayloadTooLarge(String),

    // ── 422 Unprocessable Entity ─────────────────────────────────────────────
    #[error("Validation failed")]
    ValidationError(#[from] ValidationErrors),

    // ── Extractor errors (dari Axum) ─────────────────────────────────────────
    /// JSON body tidak valid / content-type salah
    #[error("Invalid JSON body")]
    JsonError(#[from] JsonRejection),

    /// Path parameter tidak bisa di-parse (misal UUID salah format)
    #[error("Invalid path parameter")]
    PathError(#[from] PathRejection),

    /// Query string tidak bisa di-parse
    #[error("Invalid query parameter")]
    QueryError(#[from] QueryRejection),

    // ── 500 Internal Server Error ────────────────────────────────────────────
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("JWT error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Internal server error: {0}")]
    InternalError(String),
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest(_)      => StatusCode::BAD_REQUEST,
            Self::Unauthorized(_)    => StatusCode::UNAUTHORIZED,
            Self::Forbidden(_)       => StatusCode::FORBIDDEN,
            Self::NotFound(_)        => StatusCode::NOT_FOUND,
            Self::Conflict(_)        => StatusCode::CONFLICT,
            Self::PayloadTooLarge(_) => StatusCode::PAYLOAD_TOO_LARGE,
            Self::ValidationError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::JsonError(e)       => e.status(),
            Self::PathError(_)       => StatusCode::BAD_REQUEST,
            Self::QueryError(_)      => StatusCode::BAD_REQUEST,
            Self::DatabaseError(e)   => match e {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                _                        => StatusCode::INTERNAL_SERVER_ERROR,
            },
            Self::JwtError(_)        => StatusCode::UNAUTHORIZED,
            Self::IoError(_)         => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InternalError(_)   => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// Pesan yang aman dikirim ke client — detail internal tidak di-expose.
    pub fn client_message(&self) -> String {
        match self {
            Self::ValidationError(e) => {
                let messages: Vec<String> = e
                    .field_errors()
                    .iter()
                    .flat_map(|(field, errors)| {
                        errors.iter().map(move |err| {
                            let msg = err
                                .message
                                .as_ref()
                                .map(|m| m.to_string())
                                .unwrap_or_else(|| format!("Invalid value for '{}'", field));
                            format!("{}: {}", field, msg)
                        })
                    })
                    .collect();
                messages.join(", ")
            }
            Self::JsonError(e) => {
                // Berikan pesan yang helpful tapi tidak expose struktur internal
                match e {
                    JsonRejection::JsonDataError(_) =>
                        "Request body has invalid field types or missing required fields".to_string() + e.to_string().as_str(),
                    JsonRejection::JsonSyntaxError(_) =>
                        "Request body contains invalid JSON syntax".to_string(),
                    JsonRejection::MissingJsonContentType(_) =>
                        "Content-Type header must be 'application/json'".to_string(),
                    _ =>
                        "Failed to parse request body".to_string(),
                }
            }
            Self::PathError(e) => {
                format!("Invalid path parameter: {}", e.body_text())
            }
            Self::QueryError(e) => {
                format!("Invalid query parameter: {}", e.body_text())
            }
            Self::DatabaseError(e) => match e {
                sqlx::Error::RowNotFound => "Resource not found".to_string(),
                _ => {
                    tracing::error!("Database error: {:?}", e);
                    "A database error occurred".to_string()
                }
            },
            Self::IoError(e) => {
                tracing::error!("IO error: {:?}", e);
                "A file system error occurred".to_string()
            }
            Self::InternalError(msg) => {
                tracing::error!("Internal error: {}", msg);
                "An internal server error occurred".to_string()
            }
            _ => self.to_string(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status  = self.status_code();
        let message = self.client_message();

        let body = json!(Template {
            code_http: status.as_u16(),
            message,
        });

        (status, Json(body)).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;