mod routes;
mod models;
mod utils;
mod handlers;

use axum::serve;
use tokio::net::TcpListener;
use tower_http::{trace::TraceLayer, cors::CorsLayer};
use crate::routes::{init_routes};

#[derive(Clone)]
pub struct AppState {
    // Add any shared state here, e.g., database connection pool
}

#[tokio::main]
async fn main(){

    let state = AppState {
        // Initialize shared state here
    };

    let app = init_routes(state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::info!("Server running on http://0.0.0.0:3000");

    serve(listener, app).await.unwrap();
}

