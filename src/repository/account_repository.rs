use chrono::Utc;
use rust_decimal::Decimal;
use sqlx::{PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::models::Account;

// SQL query constants for CRUD operations
const QUERY_SELECT_ALL: &str = "SELECT * FROM accounts";
const QUERY_SELECT_ONE: &str = "SELECT * FROM accounts WHERE account_id = $1";
const QUERY_INSERT: &str = "
    INSERT INTO accounts (account_id, balance, created_at, updated_at) 
    VALUES ($1, $2, $3, $4) 
    RETURNING *
";
const QUERY_DELETE: &str = "DELETE FROM accounts WHERE account_id = $1";

/// Fetch all account records from the database
pub async fn get_accounts(pool: &PgPool) -> Result<Vec<Account>, sqlx::Error> {
    sqlx::query_as::<_, Account>(QUERY_SELECT_ALL)
        .fetch_all(pool)
        .await
}

/// Fetch a specific account by its ID
pub async fn get_account_by_id(pool: &PgPool, account_id: Uuid) -> Result<Account, sqlx::Error> {
    sqlx::query_as::<_, Account>(QUERY_SELECT_ONE)
        .bind(account_id)
        .fetch_one(pool)
        .await
}

/// Create a new account with the given user ID and initial balance
pub async fn create_account(
    pool: &PgPool,
    user_id: Uuid,
    balance: Decimal,
) -> Result<Account, sqlx::Error> {
    sqlx::query_as::<_, Account>(QUERY_INSERT)
        .bind(user_id)
        .bind(balance)
        .bind(Utc::now()) // created_at
        .bind(Utc::now()) // updated_at
        .fetch_one(pool)
        .await
}

/// Update the balance of an existing account
///
/// Only updates if `new_balance` is provided; otherwise, returns `RowNotFound` error
pub async fn update_account_info(
    pool: &PgPool,
    account_id: Uuid,
    new_balance: Option<Decimal>,
) -> Result<Account, sqlx::Error> {
    if new_balance.is_none() {
        return Err(sqlx::Error::RowNotFound);
    }

    let mut builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE accounts SET ");

    if let Some(new_balance) = new_balance {
        builder.push("balance = ").push_bind(new_balance);
        builder.push(", ");
    }

    // Always update the `updated_at` field
    builder.push("updated_at = ").push_bind(Utc::now());

    builder.push(" WHERE account_id = ").push_bind(account_id);
    builder.push(" RETURNING *");

    let query = builder.build_query_as::<Account>();
    let account = query.fetch_one(pool).await?;

    Ok(account)
}

/// Delete an account by ID from the database
pub async fn delete_account(pool: &PgPool, account_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(QUERY_DELETE)
        .bind(account_id)
        .execute(pool)
        .await
        .map(|_| ()) // convert result to ()
}
