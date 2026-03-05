use axum::{
    Router, 
    middleware,
    routing::{patch, get},
};

use crate::{
    AppState,
    handlers::shop_handler,
    middlewares::auth_middleware::require_auth,
};

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/top-up-money", patch(shop_handler::top_up_money))
        .route("/top-up-medal", patch(shop_handler::top_up_medal))
        .route("/list-shop-primary", get(shop_handler::list_shop_weapon_primary))
        .route_layer(middleware::from_fn_with_state(state, require_auth))
}
