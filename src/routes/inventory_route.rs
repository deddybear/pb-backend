use axum::{routing::post, Router};

use crate::{handlers::auth_handler, AppState};

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/signup", post(auth_handler::sign_up))
        .route("/login", post(auth_handler::login))
}
