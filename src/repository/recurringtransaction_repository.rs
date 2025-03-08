use crate::models::{IntervalChoices, RecurringTransaction, TransactionType};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_recurring_transactions(
    pool: &PgPool,
) -> Result<Vec<RecurringTransaction>, sqlx::Error> {
    let recurring_transactions =
        sqlx::query_as::<_, RecurringTransaction>("SELECT * FROM recurring_transactions")
            .fetch_all(pool)
            .await?;
    Ok(recurring_transactions)
}

pub async fn get_recurring_transaction_by_id(
    pool: &PgPool,
    transaction_id: Uuid,
) -> Result<RecurringTransaction, sqlx::Error> {
    let recurring_transaction = sqlx::query_as::<_, RecurringTransaction>(
        "SELECT * FROM recurring_transactions WHERE id = $1",
    )
    .bind(transaction_id)
    .fetch_one(pool)
    .await?;
    Ok(recurring_transaction)
}

pub async fn create_recurring_transaction(
    pool: &PgPool,
    account_id: Uuid,
    asset_id: Uuid,
    category_id: Option<Uuid>,
    amount: Decimal,
    interval: IntervalChoices,
    transaction_type: TransactionType,
) -> Result<RecurringTransaction, sqlx::Error> {
    let recurring_transaction = sqlx::query_as::<_, RecurringTransaction>(
        "INSERT INTO recurring_transactions (id, account_id, asset_id, category_id, amount, interval, next_execution, transaction_type, is_active, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, true, $9, $10)
         RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(account_id)
    .bind(asset_id)
    .bind(category_id)
    .bind(amount)
    .bind(interval)
    .bind(Utc::now())
    .bind(transaction_type as i32)
    .bind(Utc::now())
    .bind(Utc::now())
    .fetch_one(pool)
    .await?;

    Ok(recurring_transaction)
}

pub async fn update_recurring_transaction_info(
    pool: &PgPool,
    transaction_id: Uuid,
    amount: Option<Decimal>,
    interval: Option<IntervalChoices>,
    next_execution: Option<DateTime<Utc>>,
    is_active: Option<bool>,
) -> Result<RecurringTransaction, sqlx::Error> {
    let recurring_transaction = sqlx::query_as::<_, RecurringTransaction>(
        "UPDATE recurring_transactions SET 
            amount = COALESCE($1, amount),
            interval = COALESCE($2, interval),
            next_execution = COALESCE($3, next_execution),
            is_active = COALESCE($4, is_active),
            updated_at = $5
        WHERE id = $6 RETURNING *",
    )
    .bind(amount)
    .bind(interval.map(|i| i.to_string()))
    .bind(next_execution)
    .bind(is_active)
    .bind(Utc::now())
    .bind(transaction_id)
    .fetch_one(pool)
    .await?;

    Ok(recurring_transaction)
}

pub async fn delete_recurring_transaction(
    pool: &PgPool,
    transaction_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM recurring_transactions WHERE id = $1")
        .bind(transaction_id)
        .execute(pool)
        .await?;

    Ok(())
}
