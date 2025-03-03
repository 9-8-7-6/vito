mod models;
mod repository;
use crate::models::*;
use crate::repository::*;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    serve, Json, Router,
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let state = Arc::new(init_db().await);

    let app = Router::new()
        .route("/accounts", get(get_all_accounts))
        .route("/accounts/:id", get(get_account))
        .route("/accounts", post(add_account))
        .route("/accounts/:id", put(update_account))
        .route("/accounts/:id", delete(delete_account_handler))
        .with_state(state);

    let addr: SocketAddr = "0.0.0.0:8000".parse().unwrap();
    println!("🚀 Server running on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}

async fn init_db() -> PgPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to DB")
}

async fn get_all_accounts(State(pool): State<Arc<PgPool>>) -> Json<Vec<Account>> {
    let accounts = get_accounts(&pool).await.unwrap();
    Json(accounts)
}

async fn get_account(
    State(pool): State<Arc<PgPool>>,
    Path(account_id): Path<Uuid>,
) -> Json<Account> {
    let account = get_account_by_id(&pool, account_id).await.unwrap();
    Json(account)
}

#[derive(Deserialize)]
struct CreateAccountRequest {
    user_id: Uuid,
    balance: f64,
}

async fn add_account(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<CreateAccountRequest>,
) -> (StatusCode, Json<Account>) {
    match create_account(&pool, payload.user_id, payload.balance).await {
        Ok(account) => (StatusCode::CREATED, Json(account)),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Account {
                id: Uuid::nil(),
                user_id: Uuid::nil(),
                balance: 0.0,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            }),
        ),
    }
}

#[derive(Deserialize)]
struct UpdateAccountRequest {
    balance: f64,
}

async fn update_account(
    State(pool): State<Arc<PgPool>>,
    Path(account_id): Path<Uuid>,
    Json(payload): Json<UpdateAccountRequest>,
) -> (StatusCode, Json<Account>) {
    match update_account_balance(&pool, account_id, payload.balance).await {
        Ok(account) => (StatusCode::OK, Json(account)),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Account {
                id: Uuid::nil(),
                user_id: Uuid::nil(),
                balance: 0.0,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            }),
        ),
    }
}

async fn delete_account_handler(
    State(pool): State<Arc<PgPool>>,
    Path(account_id): Path<Uuid>,
) -> StatusCode {
    match delete_account(&pool, account_id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
