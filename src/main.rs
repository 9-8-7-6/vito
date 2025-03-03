mod db;
mod handlers;
mod models;
mod repository;
mod routes;

use axum::{serve, Router};
use routes::account_routes::account_routes;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let state = Arc::new(db::init_db().await);

    let app = Router::new().merge(account_routes(state));

    let addr: SocketAddr = "0.0.0.0:8000".parse().unwrap();
    println!("🚀 Server running on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
