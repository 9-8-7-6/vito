use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{Transaction, TransactionType};

const QUERY_SELECT_ALL: &str = "SELECT * FROM transactions";
const QUERY_SELECT_BY_USER_ID: &str =
    "SELECT * FROM transactions WHERE from_account_id = $1 OR to_account_id = $1";
const QUERY_INSERT: &str = "
    INSERT INTO transactions (
        from_asset_id, to_asset_id, transaction_type, 
        amount, fee, from_account_id, to_account_id, created_at, 
        updated_at, transaction_time, notes, image
    ) VALUES (
        $1, $2, $3, $4, $5, 
        COALESCE($6, 0.00), $7, $8, COALESCE($9, now()), 
        COALESCE($10, now()), COALESCE($11, now()), $12
    ) 
    RETURNING *
";
const QUERY_UPDATE: &str = "
    UPDATE transactions 
    SET 
        from_asset_id = COALESCE($2, from_asset_id),
        to_asset_id = COALESCE($3, to_asset_id),
        transaction_type = COALESCE($4, transaction_type),
        amount = COALESCE($5, amount),
        fee = COALESCE($6, fee),
        from_account_id = COALESCE($7, from_account_id),
        to_account_id = COALESCE($8, to_account_id),
        updated_at = now(),
        transaction_time = COALESCE($9, transaction_time),
        notes = COALESCE($10, notes),
        image = COALESCE($11, image)
    WHERE id = $1
    RETURNING *
";
const QUERY_DELETE: &str = "DELETE FROM transactions WHERE id = $1";

pub async fn get_transactions(pool: &PgPool) -> Result<Vec<Transaction>, sqlx::Error> {
    let transactions = sqlx::query_as::<_, Transaction>(QUERY_SELECT_ALL)
        .fetch_all(pool)
        .await?;
    Ok(transactions)
}

pub async fn get_transaction_by_user_id(
    pool: &PgPool,
    account_id: Uuid,
) -> Result<Vec<Transaction>, sqlx::Error> {
    sqlx::query_as::<_, Transaction>(QUERY_SELECT_BY_USER_ID)
        .bind(account_id)
        .fetch_all(pool)
        .await
}

pub async fn create_transaction(
    pool: &PgPool,
    from_asset_id: Option<Uuid>,
    to_asset_id: Option<Uuid>,
    transaction_type: TransactionType,
    amount: Decimal,
    fee: Option<Decimal>,
    from_account_id: Option<Uuid>,
    to_account_id: Option<Uuid>,
    transaction_time: Option<DateTime<Utc>>,
    notes: Option<String>,
    image: Option<String>,
) -> Result<Transaction, sqlx::Error> {
    let transaction = sqlx::query_as::<_, Transaction>(QUERY_INSERT)
        .bind(Uuid::new_v4())
        .bind(from_asset_id)
        .bind(to_asset_id)
        .bind(transaction_type as i32)
        .bind(amount)
        .bind(fee.unwrap_or(Decimal::ZERO))
        .bind(from_account_id)
        .bind(to_account_id)
        .bind(Utc::now())
        .bind(Utc::now())
        .bind(transaction_time.unwrap_or(Utc::now()))
        .bind(notes)
        .bind(image)
        .fetch_one(pool)
        .await?;

    Ok(transaction)
}

pub async fn update_transaction_info(
    pool: &PgPool,
    transaction_id: Uuid,
    from_asset_id: Option<Uuid>,
    to_asset_id: Option<Uuid>,
    transaction_type: Option<TransactionType>,
    amount: Option<Decimal>,
    fee: Option<Decimal>,
    from_account_id: Option<Uuid>,
    to_account_id: Option<Uuid>,
    transaction_time: Option<DateTime<Utc>>,
    notes: Option<String>,
    image: Option<String>,
) -> Result<Transaction, sqlx::Error> {
    let transaction = sqlx::query_as::<_, Transaction>(QUERY_UPDATE)
        .bind(transaction_id)
        .bind(from_asset_id)
        .bind(to_asset_id)
        .bind(transaction_type.map(|t| t as i32))
        .bind(amount)
        .bind(fee)
        .bind(from_account_id)
        .bind(to_account_id)
        .bind(transaction_time)
        .bind(notes)
        .bind(image)
        .fetch_one(pool)
        .await?;

    Ok(transaction)
}

pub async fn delete_transaction(pool: &PgPool, transaction_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(QUERY_DELETE)
        .bind(transaction_id)
        .execute(pool)
        .await?;

    Ok(())
}
