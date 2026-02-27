mod handlers;
mod models;
mod routes;
mod utils;
mod middlewares;

use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{routes::init_routes, utils::config::Config};

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub config: Arc<Config>,
}

#[tokio::main]
async fn main() {
    // load file .env
    dotenv::dotenv().ok();

    // Init tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("APP_DEBUG")
                .unwrap_or_else(|_| "rust_axum_api=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env().expect("Failed to load config");
    let config = Arc::new(config);

    //Database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to the database");

    let state = AppState {
        db: pool,
        config: config.clone(),
    };

    let app = init_routes(state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let addr: SocketAddr = format!("{}:{}", config.server_host, config.server_port)
        .parse()
        .expect("Failed to parse server address");

    let listener = tokio::net::TcpListener::bind(addr).await.expect("");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");

    tracing::event!(tracing::Level::INFO, "Server running on http://{}", addr);
}
