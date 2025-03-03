use crate::models::Account;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_accounts(pool: &PgPool) -> Result<Vec<Account>, sqlx::Error> {
    let accounts = sqlx::query_as::<_, Account>("SELECT * FROM accounts")
        .fetch_all(pool)
        .await?;
    Ok(accounts)
}

pub async fn get_account_by_id(pool: &PgPool, account_id: Uuid) -> Result<Account, sqlx::Error> {
    let account = sqlx::query_as::<_, Account>("SELECT * FROM accounts WHERE id = $1")
        .bind(account_id)
        .fetch_one(pool)
        .await?;

    Ok(account)
}

pub async fn create_account(
    pool: &PgPool,
    user_id: Uuid,
    balance: f64,
) -> Result<Account, sqlx::Error> {
    let account = sqlx::query_as::<_, Account>(
        "INSERT INTO accounts (id, user_id, balance, created_at, updated_at) 
        VALUES ($1, $2, $3, $4, $5) RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(user_id)
    .bind(balance)
    .bind(Utc::now())
    .bind(Utc::now())
    .fetch_one(pool)
    .await?;

    Ok(account)
}

pub async fn update_account_balance(
    pool: &PgPool,
    account_id: Uuid,
    new_balance: f64,
) -> Result<Account, sqlx::Error> {
    let account = sqlx::query_as::<_, Account>(
        "UPDATE accounts 
        SET balance = $1, updated_at = $2 
        WHERE id = $3 RETURNING *",
    )
    .bind(new_balance)
    .bind(Utc::now())
    .bind(account_id)
    .fetch_one(pool)
    .await?;

    Ok(account)
}

pub async fn delete_account(pool: &PgPool, account_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM accounts WHERE id = $1")
        .bind(account_id)
        .execute(pool)
        .await?;

    Ok(())
}
