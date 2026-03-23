use axum::{
    Router, middleware,
    routing::{get, patch},
};

use crate::{AppState, handlers::account_handler, middlewares::auth_middleware::require_auth};

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/get-data", get(account_handler::get_account))
        .route("/change-email", patch(account_handler::change_email))
        .route("/change-password", patch(account_handler::change_password))
        .route_layer(middleware::from_fn_with_state(state, require_auth))
}
