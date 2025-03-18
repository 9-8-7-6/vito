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

use crate::models::{Transaction, TransactionList, TransactionType};
use crate::repository::{
    create_transaction, delete_transaction, get_transaction_by_transation_id,
    get_transactions_by_account_id, update_transaction_info,
};

#[derive(Deserialize)]
pub struct CreateTransactionRequest {
    from_asset_id: Option<Uuid>,
    to_asset_id: Option<Uuid>,
    transaction_type: TransactionType,
    amount: Decimal,
    fee: Option<Decimal>,
    from_account_id: Option<Uuid>,
    to_account_id: Option<Uuid>,
    transaction_time: Option<chrono::DateTime<chrono::Utc>>,
    notes: Option<String>,
    image: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateTransactionRequest {
    from_asset_id: Option<Uuid>,
    to_asset_id: Option<Uuid>,
    transaction_type: Option<TransactionType>,
    amount: Option<Decimal>,
    fee: Option<Decimal>,
    from_account_id: Option<Uuid>,
    to_account_id: Option<Uuid>,
    transaction_time: Option<chrono::DateTime<chrono::Utc>>,
    notes: Option<String>,
    image: Option<String>,
}

pub async fn get_transaction_by_transaction_id_handler(
    State(pool): State<Arc<PgPool>>,
    Path(transaction_id): Path<Uuid>,
) -> impl IntoResponse {
    match get_transaction_by_transation_id(&pool, transaction_id).await {
        Ok(transaction) => transaction.into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn get_transaction_by_account_id_handler(
    State(pool): State<Arc<PgPool>>,
    Path(account_id): Path<Uuid>,
) -> impl IntoResponse {
    match get_transactions_by_account_id(&pool, account_id).await {
        Ok(transaction) => TransactionList(transaction).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn add_transaction_handler(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<CreateTransactionRequest>,
) -> impl IntoResponse {
    match create_transaction(
        &pool,
        payload.from_asset_id,
        payload.to_asset_id,
        payload.transaction_type,
        payload.amount,
        payload.fee,
        payload.from_account_id,
        payload.to_account_id,
        payload.transaction_time,
        payload.notes,
        payload.image,
    )
    .await
    {
        Ok(transaction) => transaction.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn update_transaction_handler(
    State(pool): State<Arc<PgPool>>,
    Path(transaction_id): Path<Uuid>,
    Json(payload): Json<UpdateTransactionRequest>,
) -> (StatusCode, Json<Transaction>) {
    match update_transaction_info(
        &pool,
        transaction_id,
        payload.from_asset_id,
        payload.to_asset_id,
        payload.transaction_type,
        payload.amount,
        payload.fee,
        payload.from_account_id,
        payload.to_account_id,
        payload.transaction_time,
        payload.notes,
        payload.image,
    )
    .await
    {
        Ok(transaction) => (StatusCode::OK, Json(transaction)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(dummy_transaction())),
    }
}

pub async fn delete_transaction_handler(
    State(pool): State<Arc<PgPool>>,
    Path(transaction_id): Path<Uuid>,
) -> impl IntoResponse {
    match delete_transaction(&pool, transaction_id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

fn dummy_transaction() -> Transaction {
    Transaction::default()
}
