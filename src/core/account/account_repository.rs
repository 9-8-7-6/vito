use chrono::Utc;
use rust_decimal::Decimal;
use sqlx::{PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::core::account::account::Account;

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

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    use sqlx::{migrate::MigrateDatabase, PgPool, Postgres};
    use std::env;
    use uuid::Uuid;

    /// Setup connection to test DB using `.env.test`
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
            .expect("Failed to connect to the test database");

        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        pool
    }

    /// Helper function to insert a fake user into `users` table
    async fn insert_fake_user(pool: &PgPool, user_id: Uuid) {
        let username = format!("testuser_{}", &user_id.to_string()[..8]);
        let email = format!("test_{}@example.com", &user_id.to_string()[..8]);

        let query = r#"
            INSERT INTO users (
                id, username, first_name, last_name, email, hashed_password
            ) VALUES (
                $1, $2, $3, $4, $5, $6
            )
        "#;

        sqlx::query(query)
            .bind(user_id)
            .bind(username)
            .bind("Test")
            .bind("User")
            .bind(email)
            .bind("fakehashedpassword")
            .execute(pool)
            .await
            .expect("Failed to insert fake user");
    }

    /// Delete a user by ID from the database
    pub async fn delete_user(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(user_id)
            .execute(pool)
            .await
            .map(|_| ())
    }

    #[tokio::test]
    async fn integration_test_all_account_operations() {
        let pool = setup_test_db().await;
        let user_id = Uuid::new_v4();

        insert_fake_user(&pool, user_id).await;

        // Create a new account
        let initial_balance = Decimal::new(1000, 2); // 10.00
        let account = create_account(&pool, user_id, initial_balance)
            .await
            .expect("create_account failed");
        assert_eq!(account.account_id, user_id);
        assert_eq!(account.balance, initial_balance);

        // Read the account back
        let fetched = get_account_by_id(&pool, user_id)
            .await
            .expect("get_account_by_id failed");
        assert_eq!(fetched.account_id, user_id);
        assert_eq!(fetched.balance, initial_balance);

        // Update account balance
        let updated_balance = Decimal::new(7500, 2); // 75.00
        let updated = update_account_info(&pool, user_id, Some(updated_balance))
            .await
            .expect("update_account_info failed");
        assert_eq!(updated.balance, updated_balance);

        // Attempt to update with None (should fail)
        let result = update_account_info(&pool, user_id, None).await;
        assert!(matches!(result, Err(sqlx::Error::RowNotFound)));

        // Delete account
        delete_account(&pool, user_id)
            .await
            .expect("delete_account failed");

        let result = get_account_by_id(&pool, user_id).await;
        assert!(matches!(result, Err(sqlx::Error::RowNotFound)));

        // Delete user to clean up
        delete_user(&pool, user_id)
            .await
            .expect("delete_user failed");
    }
}
