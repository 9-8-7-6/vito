use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use rust_decimal::Decimal;
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::models::AccountList;
use crate::repository::{
    create_account, delete_account, get_account_by_id, get_accounts, update_account_info,
};

#[derive(Deserialize)]
pub struct CreateAccountRequest {
    user_id: Uuid,
    balance: Decimal,
}

#[derive(Deserialize)]
pub struct UpdateAccountRequest {
    balance: Decimal,
}

pub async fn get_all_accounts_handler(State(pool): State<Arc<PgPool>>) -> impl IntoResponse {
    match get_accounts(&pool).await {
        Ok(accounts) => AccountList(accounts).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_account_handler(
    State(pool): State<Arc<PgPool>>,
    Path(account_id): Path<Uuid>,
) -> impl IntoResponse {
    match get_account_by_id(&pool, account_id).await {
        Ok(account) => account.into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn add_account_handler(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<CreateAccountRequest>,
) -> impl IntoResponse {
    match create_account(&pool, payload.user_id, payload.balance).await {
        Ok(account) => account.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn update_account_handler(
    State(pool): State<Arc<PgPool>>,
    Path(account_id): Path<Uuid>,
    Json(payload): Json<UpdateAccountRequest>,
) -> impl IntoResponse {
    match update_account_info(&pool, account_id, payload.balance).await {
        Ok(account) => account.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn delete_account_handler(
    State(pool): State<Arc<PgPool>>,
    Path(account_id): Path<Uuid>,
) -> impl IntoResponse {
    match delete_account(&pool, account_id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
