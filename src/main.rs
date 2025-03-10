mod db;
mod error;
mod handlers;
mod models;
mod repository;
mod routes;

use axum::{serve, Router};
use dotenvy::dotenv;
use routes::account_routes::account_routes;
use routes::asset_routes::asset_routes;
use routes::category_routes::category_routes;
use routes::login_logout_routes::login_routes;
use routes::recurring_transaction_routes::recurringtransaction_routes;
use routes::transaction_routes::transaction_routes;
use routes::user_routes::user_routes;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let state = Arc::new(db::init_db().await);
    let session_layer = db::init_redis().await;

    let routes_all = Router::new()
        .merge(account_routes(state.clone()))
        .merge(user_routes(state.clone()))
        .merge(asset_routes(state.clone()))
        .merge(category_routes(state.clone()))
        .merge(recurringtransaction_routes(state.clone()))
        .merge(transaction_routes(state.clone()))
        .merge(login_routes(state.clone()))
        .layer(CookieManagerLayer::new())
        .layer(session_layer);

    let addr: SocketAddr = "0.0.0.0:8000".parse().unwrap();
    println!("🚀 Server running on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, routes_all).await.unwrap();
}
