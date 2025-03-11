mod db;
mod handlers;
mod models;
mod repository;
mod routes;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{serve, Router};
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;

use crate::models::Backend;

use routes::{
    account_routes::account_routes, asset_routes::asset_routes, category_routes::category_routes,
    login_logout_routes::login_routes, recurring_transaction_routes::recurringtransaction_routes,
    transaction_routes::transaction_routes, user_routes::user_routes,
};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let state = Arc::new(db::init_db().await);
    let backend = Backend::new(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let session_layer = db::init_redis().await;

    let routes_all = Router::new()
        .merge(account_routes(state.clone()))
        .merge(user_routes(state.clone()))
        .merge(asset_routes(state.clone()))
        .merge(category_routes(state.clone()))
        .merge(recurringtransaction_routes(state.clone()))
        .merge(transaction_routes(state.clone()))
        .merge(login_routes(backend.clone()))
        .layer(CookieManagerLayer::new())
        .layer(session_layer);

    let addr: SocketAddr = "0.0.0.0:8000".parse().unwrap();
    println!("🚀 Server running on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, routes_all).await.unwrap();
}
