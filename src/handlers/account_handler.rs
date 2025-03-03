use crate::models::Account;
use crate::repository::{
    create_account, delete_account, get_account_by_id, get_accounts, update_account_balance,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub async fn get_all_accounts(State(pool): State<Arc<PgPool>>) -> Json<Vec<Account>> {
    let accounts = get_accounts(&pool).await.unwrap();
    Json(accounts)
}

pub async fn get_account(
    State(pool): State<Arc<PgPool>>,
    Path(account_id): Path<Uuid>,
) -> Json<Account> {
    let account = get_account_by_id(&pool, account_id).await.unwrap();
    Json(account)
}

#[derive(Deserialize)]
pub struct CreateAccountRequest {
    user_id: Uuid,
    balance: f64,
}

pub async fn add_account(
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
pub struct UpdateAccountRequest {
    balance: f64,
}

pub async fn update_account(
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

pub async fn delete_account_handler(
    State(pool): State<Arc<PgPool>>,
    Path(account_id): Path<Uuid>,
) -> StatusCode {
    match delete_account(&pool, account_id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
