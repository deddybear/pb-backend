use axum::{
    Router, middleware,
    routing::{get, patch},
};

use crate::{AppState, handlers::shop_handler, middlewares::auth_middleware::require_auth};

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/top-up-money", patch(shop_handler::top_up_money))
        .route("/top-up-medal", patch(shop_handler::top_up_medal))
        .route(
            "/list-shop-primary",
            get(shop_handler::list_shop_weapon_primary),
        )
        .route(
            "/list-shop-secondary",
            get(shop_handler::list_shop_weapon_secondary),
        )
        .route(
            "/list-shop-melee",
            get(shop_handler::list_shop_weapon_melee),
        )
        .route(
            "/list-shop-explosive",
            get(shop_handler::list_shop_weapon_explosive),
        )
        .route(
            "/list-shop-character",
            get(shop_handler::list_shop_character),
        )
        .route(
            "/list-shop-headgear",
            get(shop_handler::list_shop_headgear),
        )
        .route(
            "/list-shop-consume",
            get(shop_handler::list_shop_consume),
        )
        .route_layer(middleware::from_fn_with_state(state, require_auth))
}
