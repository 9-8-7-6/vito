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

use crate::models::{
    EnrichedTransaction, EnrichedTransactionList, Transaction, TransactionList, TransactionType,
};
use crate::repository::{
    create_transaction, delete_transaction, get_transaction_by_transation_id,
    get_transactions_by_account_id, update_asset_balance, update_transaction_info,
};

/// Payload for creating a new transaction
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

/// Payload for updating an existing transaction
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

/// Handler: Get a single transaction by its ID
pub async fn get_transaction_by_transaction_id_handler(
    State(pool): State<Arc<PgPool>>,
    Path(transaction_id): Path<Uuid>,
) -> impl IntoResponse {
    match get_transaction_by_transation_id(&pool, transaction_id).await {
        Ok(transaction) => transaction.into_response(),
        Err(err) => {
            eprintln!("Failed to fetch transaction {}: {:?}", transaction_id, err);
            StatusCode::NOT_FOUND.into_response()
        }
    }
}

/// Handler: Get all transactions associated with a given account ID
pub async fn get_transaction_by_account_id_handler(
    State(pool): State<Arc<PgPool>>,
    Path(account_id): Path<Uuid>,
) -> impl IntoResponse {
    match get_transactions_by_account_id(&pool, account_id).await {
        Ok(tx) => EnrichedTransactionList(tx).into_response(),
        Err(err) => {
            eprintln!(
                "Failed to fetch transactions by account {}: {:?}",
                account_id, err
            );
            StatusCode::NOT_FOUND.into_response()
        }
    }
}

/// Handler: Create a new transaction and update the asset balances accordingly
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
        Ok(transaction) => {
            // Apply balance to `to_asset`
            if let Some(to_asset_id) = payload.to_asset_id {
                if let Err(e) = update_asset_balance(&pool, to_asset_id, payload.amount).await {
                    eprintln!("Failed to update to_asset balance: {:?}", e);
                }
            }

            // Deduct from `from_asset`, including fee
            if let Some(from_asset_id) = payload.from_asset_id {
                let mut offset = payload.amount;
                if let Some(fee) = payload.fee {
                    offset += fee;
                }
                if let Err(e) = update_asset_balance(&pool, from_asset_id, -offset).await {
                    eprintln!("Failed to update from_asset balance: {:?}", e);
                }
            }

            transaction.into_response()
        }
        Err(err) => {
            eprintln!("Failed to create transaction: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Handler: Update an existing transaction and rollback/reapply its asset balance
pub async fn update_transaction_handler(
    State(pool): State<Arc<PgPool>>,
    Path(transaction_id): Path<Uuid>,
    Json(payload): Json<UpdateTransactionRequest>,
) -> (StatusCode, Json<Transaction>) {
    // Step 1: Load existing transaction for rollback
    let old_transaction = match get_transaction_by_transation_id(&pool, transaction_id).await {
        Ok(tx) => tx,
        Err(err) => {
            eprintln!(
                "Failed to fetch transaction before update {}: {:?}",
                transaction_id, err
            );
            return (StatusCode::NOT_FOUND, Json(dummy_transaction()));
        }
    };

    // Step 2: Revert old balance effects
    if let Some(to_asset_id) = old_transaction.to_asset_id {
        if let Err(e) = update_asset_balance(&pool, to_asset_id, -old_transaction.amount).await {
            eprintln!("Failed to revert to_asset balance: {:?}", e);
        }
    }

    if let Some(from_asset_id) = old_transaction.from_asset_id {
        let mut offset = old_transaction.amount + old_transaction.fee;
        if let Err(e) = update_asset_balance(&pool, from_asset_id, offset).await {
            eprintln!("Failed to revert from_asset balance: {:?}", e);
        }
    }

    // Step 3: Apply new update
    let updated_transaction = match update_transaction_info(
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
        Ok(tx) => tx,
        Err(err) => {
            eprintln!("Failed to update transaction {}: {:?}", transaction_id, err);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(dummy_transaction()));
        }
    };

    // Step 4: Apply new balance effects
    if let Some(to_asset_id) = updated_transaction.to_asset_id {
        if let Err(e) = update_asset_balance(&pool, to_asset_id, updated_transaction.amount).await {
            eprintln!("Failed to apply new to_asset balance: {:?}", e);
        }
    }

    if let Some(from_asset_id) = updated_transaction.from_asset_id {
        let mut offset = updated_transaction.amount + updated_transaction.fee;
        if let Err(e) = update_asset_balance(&pool, from_asset_id, -offset).await {
            eprintln!("Failed to apply new from_asset balance: {:?}", e);
        }
    }

    (StatusCode::OK, Json(updated_transaction))
}

/// Handler: Delete a transaction and roll back its asset balance changes
pub async fn delete_transaction_handler(
    State(pool): State<Arc<PgPool>>,
    Path(transaction_id): Path<Uuid>,
) -> impl IntoResponse {
    // Step 1: Load transaction before deleting
    let old_transaction = match get_transaction_by_transation_id(&pool, transaction_id).await {
        Ok(tx) => tx,
        Err(err) => {
            eprintln!(
                "Transaction {} not found, delete skipped: {:?}",
                transaction_id, err
            );
            return StatusCode::NO_CONTENT.into_response();
        }
    };

    // Step 2: Roll back balance effects
    if let Some(to_asset_id) = old_transaction.to_asset_id {
        if let Err(e) = update_asset_balance(&pool, to_asset_id, -old_transaction.amount).await {
            eprintln!("Failed to revert to_asset balance on delete: {:?}", e);
        }
    }

    if let Some(from_asset_id) = old_transaction.from_asset_id {
        let mut offset = old_transaction.amount + old_transaction.fee;
        if let Err(e) = update_asset_balance(&pool, from_asset_id, offset).await {
            eprintln!("Failed to revert from_asset balance on delete: {:?}", e);
        }
    }

    // Step 3: Delete transaction record
    match delete_transaction(&pool, transaction_id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => {
            eprintln!("Failed to delete transaction {}: {:?}", transaction_id, err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Fallback for failed update/insert operations
fn dummy_transaction() -> Transaction {
    Transaction::default()
}
