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

// Request body format for creating an account
#[derive(Deserialize)]
pub struct CreateAccountRequest {
    pub user_id: Uuid,
    pub balance: Decimal,
}

// Request body format for updating an account
#[derive(Deserialize)]
pub struct UpdateAccountRequest {
    pub balance: Option<Decimal>,
}

/// Handler to retrieve all accounts
pub async fn get_all_accounts_handler(State(pool): State<Arc<PgPool>>) -> impl IntoResponse {
    match get_accounts(&pool).await {
        Ok(accounts) => AccountList(accounts).into_response(),
        Err(err) => {
            eprintln!("Failed to fetch accounts: {:#?}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Handler to retrieve a single account by its ID
pub async fn get_account_handler(
    State(pool): State<Arc<PgPool>>,
    Path(account_id): Path<Uuid>,
) -> impl IntoResponse {
    match get_account_by_id(&pool, account_id).await {
        Ok(account) => account.into_response(),
        Err(err) => {
            eprintln!("Failed to fetch account {}: {:#?}", account_id, err);
            StatusCode::NOT_FOUND.into_response()
        }
    }
}

/// Handler to create a new account
pub async fn add_account_handler(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<CreateAccountRequest>,
) -> impl IntoResponse {
    match create_account(&pool, payload.user_id, payload.balance).await {
        Ok(account) => account.into_response(),
        Err(err) => {
            eprintln!(
                "Failed to create account for user {}: {:#?}",
                payload.user_id, err
            );
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Handler to update an existing account's balance
pub async fn update_account_handler(
    State(pool): State<Arc<PgPool>>,
    Path(account_id): Path<Uuid>,
    Json(payload): Json<UpdateAccountRequest>,
) -> impl IntoResponse {
    match update_account_info(&pool, account_id, payload.balance).await {
        Ok(account) => account.into_response(),
        Err(err) => {
            eprintln!("Failed to update account {}: {:#?}", account_id, err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Handler to delete an account by its ID
pub async fn delete_account_handler(
    State(pool): State<Arc<PgPool>>,
    Path(account_id): Path<Uuid>,
) -> impl IntoResponse {
    match delete_account(&pool, account_id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(), // 204 No Content
        Err(err) => {
            eprintln!("Failed to delete account {}: {:#?}", account_id, err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
