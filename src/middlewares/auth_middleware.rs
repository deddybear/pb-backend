use axum::{
    body::Body,
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::IntoResponse,
};

use base64::{Engine, prelude::BASE64_STANDARD};

use crate::{AppState, models::auth_model::AuthUser, utils::{errors::AppError, jwt::verify_token}};



/// # Fungsi middleware untuk pengecheckan apakah sudah login atau belum
pub async fn require_auth(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".into()))?;

    let token_base64 = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
        AppError::Unauthorized("Invalid token format. Use: Bearer <token>".into())
    })?;

    let token_bytes = BASE64_STANDARD.decode(token_base64).unwrap();

    let token = String::from_utf8(token_bytes).unwrap();

    let claims = verify_token(&token, &state.config.jwt_secret)
        .map_err(|_| AppError::Unauthorized("Invalid or expired token".into()))?;


    req.extensions_mut().insert(AuthUser {
        id: claims.sub,
        email: claims.email,
        access_level: claims.access_level,
    });

    Ok(next.run(req).await)
}
