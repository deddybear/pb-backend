use axum::{
    Router, 
    middleware,
    routing::{patch}
};

use crate::{
    middlewares::auth_middleware::require_auth,
    handlers::auth_handler, 
    AppState
};

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
    .route("/update", patch(auth_handler::sign_up))
    .route("/login", patch(auth_handler::login))
    .route_layer(middleware::from_fn_with_state(state, require_auth))
}
