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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{IntervalChoices, RecurringTransactionType};
    use rust_decimal::Decimal;
    use sqlx::{migrate::MigrateDatabase, PgPool, Postgres};
    use std::env;
    use uuid::Uuid;

    async fn setup_test_db() -> PgPool {
        dotenvy::from_filename(".env.test").ok();
        let test_database_url =
            env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env.test");

        if !Postgres::database_exists(&test_database_url)
            .await
            .unwrap_or(false)
        {
            Postgres::create_database(&test_database_url)
                .await
                .expect("Failed to create test database");
        }

        let pool = PgPool::connect(&test_database_url)
            .await
            .expect("Failed to connect to test DB");

        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        pool
    }

    async fn setup_account_and_asset(pool: &PgPool, user_id: Uuid) -> (Uuid, Uuid) {
        // Insert test user
        sqlx::query(
            "INSERT INTO users (id, username, email, hashed_password) VALUES ($1, $2, $3, $4)",
        )
        .bind(user_id)
        .bind(format!("user_{}", &user_id.to_string()[..8]))
        .bind(format!("{}@example.com", &user_id.to_string()[..8]))
        .bind("hashed")
        .execute(pool)
        .await
        .unwrap();

        // Insert account
        sqlx::query("INSERT INTO accounts (account_id, balance, created_at, updated_at) VALUES ($1, $2, now(), now())")
            .bind(user_id)
            .bind(Decimal::new(10000, 2))
            .execute(pool)
            .await
            .unwrap();

        let asset_id = Uuid::new_v4();
        sqlx::query("INSERT INTO assets (id, account_id, asset_type, balance, created_at, updated_at) VALUES ($1, $2, $3, $4, now(), now())")
            .bind(asset_id)
            .bind(user_id)
            .bind("bank")
            .bind(Decimal::new(5000, 2))
            .execute(pool)
            .await
            .unwrap();

        (user_id, asset_id)
    }

    #[tokio::test]
    async fn integration_test_recurring_transaction_crud() {
        let pool = setup_test_db().await;
        let user_id = Uuid::new_v4();
        let (account_id, asset_id) = setup_account_and_asset(&pool, user_id).await;

        let created = create_recurring_transaction(
            &pool,
            account_id,
            asset_id,
            Decimal::new(1500, 2),
            IntervalChoices::Monthly,
            RecurringTransactionType::Expense,
        )
        .await
        .unwrap();

        assert_eq!(created.account_id, account_id);
        assert_eq!(created.asset_id, asset_id);
        assert_eq!(created.amount, Decimal::new(1500, 2));

        let fetched = get_recurring_transaction_by_id(&pool, created.id)
            .await
            .unwrap();
        assert_eq!(fetched.id, created.id);

        let updated = update_recurring_transaction_info(
            &pool,
            created.id,
            Some(Decimal::new(2000, 2)),
            Some(IntervalChoices::Weekly),
            None,
            Some(false),
        )
        .await
        .unwrap();

        assert_eq!(updated.amount, Decimal::new(2000, 2));
        assert_eq!(updated.interval, IntervalChoices::Weekly);
        assert_eq!(updated.is_active, false);

        delete_recurring_transaction(&pool, created.id)
            .await
            .expect("Failed to delete transaction");

        let result = get_recurring_transaction_by_id(&pool, created.id).await;
        assert!(matches!(result, Err(sqlx::Error::RowNotFound)));
    }
}
