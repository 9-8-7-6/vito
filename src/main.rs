use axum::{extract::State, routing::get, Json, Router};
use dotenvy::dotenv;
use serde::Serialize;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;

mod models;
mod repository;

#[derive(Clone)]
struct AppState {
    db_pool: Arc<PgPool>,
}

#[tokio::main]
async fn main() {
    let state = init_db().await;

    let app = Router::new()
        .route("/", get(|| async { "Hello, Axum!" }))
        .route("/health", get(health_check))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Server running on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn init_db() -> AppState {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to DB");

    AppState {
        db_pool: Arc::new(pool),
    }
}

async fn health_check() -> &'static str {
    "OK"
}
