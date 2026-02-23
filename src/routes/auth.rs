use axum::{routing::post, Router};

use crate::{handlers::auth, AppState};

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/signup", post(auth::sign_up))
        .route("/login", post(auth::login))
}
