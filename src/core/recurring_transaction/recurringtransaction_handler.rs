use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::models::{IntervalChoices, RecurringTransaction, RecurringTransactionType};
use crate::repository::{
    create_recurring_transaction, delete_recurring_transaction, get_recurring_transaction_by_id,
    get_recurring_transactions, update_recurring_transaction_info,
};

/// Request body for creating a recurring transaction
#[derive(Deserialize)]
pub struct CreateRecurringTransactionRequest {
    pub account_id: Uuid,
    pub asset_id: Uuid,
    pub amount: Decimal,
    pub interval: IntervalChoices,
    pub transaction_type: RecurringTransactionType,
}

/// Request body for updating a recurring transaction
#[derive(Deserialize)]
pub struct UpdateRecurringTransactionRequest {
    pub amount: Option<Decimal>,
    pub interval: Option<IntervalChoices>,
    pub next_execution: Option<DateTime<Utc>>,
    pub is_active: Option<bool>,
}

/// Handler: Fetch all recurring transactions from the database
pub async fn get_all_recurring_transactions_handler(
    State(pool): State<Arc<PgPool>>,
) -> Json<Vec<RecurringTransaction>> {
    let transactions = get_recurring_transactions(&pool).await.unwrap();
    Json(transactions)
}

/// Handler: Fetch a specific recurring transaction by ID
pub async fn get_recurring_transaction_handler(
    State(pool): State<Arc<PgPool>>,
    Path(transaction_id): Path<Uuid>,
) -> Json<RecurringTransaction> {
    let transaction = get_recurring_transaction_by_id(&pool, transaction_id)
        .await
        .unwrap();
    Json(transaction)
}

/// Handler: Create a new recurring transaction
pub async fn add_recurring_transaction_handler(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<CreateRecurringTransactionRequest>,
) -> (StatusCode, Json<RecurringTransaction>) {
    match create_recurring_transaction(
        &pool,
        payload.account_id,
        payload.asset_id,
        payload.amount,
        payload.interval,
        payload.transaction_type,
    )
    .await
    {
        Ok(transaction) => (StatusCode::CREATED, Json(transaction)),
        Err(err) => {
            eprintln!("Failed to create recurring transaction: {:#?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(dummy_recurring_transaction()), // fallback dummy object
            )
        }
    }
}

/// Handler: Update an existing recurring transaction
pub async fn update_recurring_transaction_handler(
    State(pool): State<Arc<PgPool>>,
    Path(transaction_id): Path<Uuid>,
    Json(payload): Json<UpdateRecurringTransactionRequest>,
) -> (StatusCode, Json<RecurringTransaction>) {
    match update_recurring_transaction_info(
        &pool,
        transaction_id,
        payload.amount,
        payload.interval,
        payload.next_execution,
        payload.is_active,
    )
    .await
    {
        Ok(transaction) => (StatusCode::OK, Json(transaction)),
        Err(err) => {
            eprintln!(
                "Failed to update recurring transaction {}: {:#?}",
                transaction_id, err
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(dummy_recurring_transaction()), // fallback dummy object
            )
        }
    }
}

/// Handler: Delete a recurring transaction by ID
pub async fn delete_recurring_transaction_handler(
    State(pool): State<Arc<PgPool>>,
    Path(transaction_id): Path<Uuid>,
) -> StatusCode {
    match delete_recurring_transaction(&pool, transaction_id).await {
        Ok(_) => StatusCode::NO_CONTENT, // 204: Successfully deleted
        Err(err) => {
            eprintln!(
                "Failed to delete recurring transaction {}: {:#?}",
                transaction_id, err
            );
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

/// Dummy object used when transaction creation/update fails
fn dummy_recurring_transaction() -> RecurringTransaction {
    RecurringTransaction {
        id: Uuid::nil(),
        account_id: Uuid::nil(),
        asset_id: Uuid::nil(),
        amount: Decimal::ZERO,
        interval: IntervalChoices::Daily,
        next_execution: Utc::now(),
        transaction_type: RecurringTransactionType::Income,
        is_active: false,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}
