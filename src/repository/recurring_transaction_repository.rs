use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::{PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::models::{IntervalChoices, RecurringTransaction, RecurringTransactionType};

// SQL queries
const QUERY_SELECT_ALL: &str = "SELECT * FROM recurring_transactions";
const QUERY_SELECT_ONE: &str = "SELECT * FROM recurring_transactions WHERE id = $1";
const QUERY_INSERT: &str = "
    INSERT INTO recurring_transactions (
        id, account_id, asset_id, amount, interval, 
        next_execution, transaction_type, is_active, created_at, updated_at
    ) VALUES (
        $1, $2, $3, $4, $5, $6, $7, true, $9, $8
    )
    RETURNING *
";
const QUERY_DELETE: &str = "DELETE FROM recurring_transactions WHERE id = $1";

/// Fetch all recurring transactions from the database
pub async fn get_recurring_transactions(
    pool: &PgPool,
) -> Result<Vec<RecurringTransaction>, sqlx::Error> {
    let recurring_transactions = sqlx::query_as::<_, RecurringTransaction>(QUERY_SELECT_ALL)
        .fetch_all(pool)
        .await?;
    Ok(recurring_transactions)
}

/// Fetch a single recurring transaction by its ID
pub async fn get_recurring_transaction_by_id(
    pool: &PgPool,
    transaction_id: Uuid,
) -> Result<RecurringTransaction, sqlx::Error> {
    let recurring_transaction = sqlx::query_as::<_, RecurringTransaction>(QUERY_SELECT_ONE)
        .bind(transaction_id)
        .fetch_one(pool)
        .await?;
    Ok(recurring_transaction)
}

/// Create a new recurring transaction with the provided details
pub async fn create_recurring_transaction(
    pool: &PgPool,
    account_id: Uuid,
    asset_id: Uuid,
    amount: Decimal,
    interval: IntervalChoices,
    transaction_type: RecurringTransactionType,
) -> Result<RecurringTransaction, sqlx::Error> {
    let recurring_transaction = sqlx::query_as::<_, RecurringTransaction>(QUERY_INSERT)
        .bind(Uuid::new_v4()) // id
        .bind(account_id)
        .bind(asset_id)
        .bind(amount)
        .bind(interval)
        .bind(Utc::now()) // next_execution (default to now)
        .bind(transaction_type as i32)
        .bind(Utc::now()) // updated_at
        .bind(Utc::now()) // created_at
        .fetch_one(pool)
        .await?;

    Ok(recurring_transaction)
}

/// Update fields of a recurring transaction such as amount, interval, execution time, or active status
pub async fn update_recurring_transaction_info(
    pool: &PgPool,
    transaction_id: Uuid,
    amount: Option<Decimal>,
    interval: Option<IntervalChoices>,
    next_execution: Option<DateTime<Utc>>,
    is_active: Option<bool>,
) -> Result<RecurringTransaction, sqlx::Error> {
    // If no fields are provided to update, return an error
    if amount.is_none() && interval.is_none() && next_execution.is_none() && is_active.is_none() {
        return Err(sqlx::Error::RowNotFound);
    }

    let mut builder: QueryBuilder<Postgres> =
        QueryBuilder::new("UPDATE recurring_transactions SET ");

    if let Some(amount) = amount {
        builder.push("amount = ").push_bind(amount);
        builder.push(", ");
    }

    if let Some(interval) = interval {
        builder.push("interval = ").push_bind(interval);
        builder.push(", ");
    }

    if let Some(next_execution) = next_execution {
        builder.push("next_execution = ").push_bind(next_execution);
        builder.push(", ");
    }

    if let Some(is_active) = is_active {
        builder.push("is_active = ").push_bind(is_active);
        builder.push(", ");
    }

    // Always update the timestamp
    builder.push("updated_at = ").push_bind(Utc::now());

    builder.push(" WHERE id = ").push_bind(transaction_id);
    builder.push(" RETURNING *");

    let query = builder.build_query_as::<RecurringTransaction>();
    let recurring_transaction = query.fetch_one(pool).await?;

    Ok(recurring_transaction)
}

/// Delete a recurring transaction by its ID
pub async fn delete_recurring_transaction(
    pool: &PgPool,
    transaction_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query(QUERY_DELETE)
        .bind(transaction_id)
        .execute(pool)
        .await?;

    Ok(())
}
