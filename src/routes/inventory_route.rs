use axum::{
    Router, middleware,
    routing::{get, delete},
};

use crate::{AppState, handlers::inventory_handler, middlewares::auth_middleware::require_auth};

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/get-list", get(inventory_handler::get_list))
        .route("/get-data", get(inventory_handler::get_data))
        .route("/delete-item", delete(inventory_handler::delete))
        // .route("/top-up-money", patch(inventory_handler::top_up_money))
        // .route("/top-up-medal", patch(inventory_handler::top_up_medal))
        .route_layer(middleware::from_fn_with_state(state, require_auth))
}
