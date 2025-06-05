use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::{PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::models::{EnrichedTransaction, Transaction, TransactionType};
use crate::repository::asset_repository::get_asset_type_by_asset_id;

// SQL query to get all transactions where the given account is either sender or receiver
const QUERY_SELECT_BY_ACCOUNT_ID: &str =
    "SELECT * FROM transactions WHERE from_account_id = $1 OR to_account_id = $1";

// SQL query to fetch a single transaction by ID
const QUERY_SELECT_BY_TRANSACTION_ID: &str = "SELECT * FROM transactions WHERE id = $1";

// SQL insert query for creating a new transaction
const QUERY_INSERT: &str = "
    INSERT INTO transactions (
        from_asset_id, to_asset_id, transaction_type,
        amount, fee, from_account_id, to_account_id,
        created_at, updated_at, transaction_time, notes, image
    ) VALUES (
        $1, $2, $3, $4, $5, $6, $7,
        $8, $9, $10, $11, $12
    )
    RETURNING *
";

// SQL query to delete a transaction by ID
const QUERY_DELETE: &str = "DELETE FROM transactions WHERE id = $1";

/// Get all transactions involving a specific account and enrich them with asset type names.
pub async fn get_transactions_by_account_id(
    pool: &PgPool,
    account_id: Uuid,
) -> Result<Vec<EnrichedTransaction>, sqlx::Error> {
    let transactions = sqlx::query_as::<_, Transaction>(QUERY_SELECT_BY_ACCOUNT_ID)
        .bind(account_id)
        .fetch_all(pool)
        .await?;

    let mut enriched = Vec::with_capacity(transactions.len());

    for tx in transactions {
        // Get asset type for from_asset_id (if exists)
        let from_asset_type = match tx.from_asset_id {
            Some(asset_id) => Some(get_asset_type_by_asset_id(pool, asset_id).await?),
            None => None,
        };

        // Get asset type for to_asset_id (if exists)
        let to_asset_type = match tx.to_asset_id {
            Some(asset_id) => Some(get_asset_type_by_asset_id(pool, asset_id).await?),
            None => None,
        };

        // Combine original transaction with enriched asset type info
        enriched.push(EnrichedTransaction {
            id: tx.id,
            from_asset_id: tx.from_asset_id,
            to_asset_id: tx.to_asset_id,
            transaction_type: tx.transaction_type,
            amount: tx.amount,
            fee: tx.fee,
            from_account_id: tx.from_account_id,
            to_account_id: tx.to_account_id,
            created_at: tx.created_at,
            updated_at: tx.updated_at,
            transaction_time: tx.transaction_time,
            notes: tx.notes,
            image: tx.image,
            from_asset_type,
            to_asset_type,
        });
    }

    Ok(enriched)
}

/// Get a transaction by its ID
pub async fn get_transaction_by_transation_id(
    pool: &PgPool,
    transaction_id: Uuid,
) -> Result<Transaction, sqlx::Error> {
    sqlx::query_as::<_, Transaction>(QUERY_SELECT_BY_TRANSACTION_ID)
        .bind(transaction_id)
        .fetch_one(pool)
        .await
}

/// Create a new transaction
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
    match sqlx::query_as::<_, Transaction>(QUERY_INSERT)
        .bind(from_asset_id)
        .bind(to_asset_id)
        .bind(transaction_type as i32)
        .bind(amount)
        .bind(fee.unwrap_or(Decimal::ZERO)) // Use zero fee if not provided
        .bind(from_account_id)
        .bind(to_account_id)
        .bind(Utc::now()) // created_at
        .bind(Utc::now()) // updated_at
        .bind(transaction_time.unwrap_or(Utc::now())) // default to now if missing
        .bind(notes)
        .bind(image)
        .fetch_one(pool)
        .await
    {
        Ok(transaction) => {
            println!("Transaction created successfully: {:?}", transaction);
            Ok(transaction)
        }
        Err(err) => {
            eprintln!("Failed to create transaction: {}", err);
            Err(err)
        }
    }
}

/// Update one or more fields of a transaction
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
    // If no fields are being updated, return an error
    if from_asset_id.is_none()
        && to_asset_id.is_none()
        && transaction_type.is_none()
        && amount.is_none()
        && fee.is_none()
        && from_account_id.is_none()
        && to_account_id.is_none()
        && transaction_time.is_none()
        && notes.is_none()
        && image.is_none()
    {
        return Err(sqlx::Error::RowNotFound);
    }

    let mut builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE transactions SET ");

    // Conditionally push each field if it was provided
    if let Some(from_asset_id) = from_asset_id {
        builder
            .push("from_asset_id = ")
            .push_bind(from_asset_id)
            .push(", ");
    }
    if let Some(to_asset_id) = to_asset_id {
        builder
            .push("to_asset_id = ")
            .push_bind(to_asset_id)
            .push(", ");
    }
    if let Some(transaction_type) = transaction_type {
        builder
            .push("transaction_type = ")
            .push_bind(transaction_type)
            .push(", ");
    }
    if let Some(amount) = amount {
        builder.push("amount = ").push_bind(amount).push(", ");
    }
    if let Some(fee) = fee {
        builder.push("fee = ").push_bind(fee).push(", ");
    }
    if let Some(from_account_id) = from_account_id {
        builder
            .push("from_account_id = ")
            .push_bind(from_account_id)
            .push(", ");
    }
    if let Some(to_account_id) = to_account_id {
        builder
            .push("to_account_id = ")
            .push_bind(to_account_id)
            .push(", ");
    }
    if let Some(transaction_time) = transaction_time {
        builder
            .push("transaction_time = ")
            .push_bind(transaction_time)
            .push(", ");
    }
    if let Some(notes) = notes {
        builder.push("notes = ").push_bind(notes).push(", ");
    }
    if let Some(image) = image {
        builder.push("image = ").push_bind(image).push(", ");
    }

    // Always update the timestamp
    builder.push("updated_at = ").push_bind(Utc::now());
    builder.push(" WHERE id = ").push_bind(transaction_id);
    builder.push(" RETURNING *");

    // Execute and return the updated transaction
    let query = builder.build_query_as::<Transaction>();
    let transaction = query.fetch_one(pool).await?;

    Ok(transaction)
}

/// Delete a transaction by its ID
pub async fn delete_transaction(pool: &PgPool, transaction_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(QUERY_DELETE)
        .bind(transaction_id)
        .execute(pool)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use rust_decimal::Decimal;
    use sqlx::{migrate::MigrateDatabase, PgPool, Postgres};
    use std::env;
    use uuid::Uuid;

    async fn setup_test_db() -> PgPool {
        dotenvy::from_filename(".env.test").ok();
        let test_database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

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
            .expect("Failed to connect to DB");

        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("Migrations failed");

        pool
    }

    async fn insert_user_and_account(pool: &PgPool) -> Uuid {
        let user_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO users (id, username, email, hashed_password) VALUES ($1, $2, $3, $4)",
        )
        .bind(user_id)
        .bind(format!("user_{}", &user_id.to_string()[..8]))
        .bind(format!("{}@test.com", &user_id.to_string()[..8]))
        .bind("hashed_pw")
        .execute(pool)
        .await
        .unwrap();

        sqlx::query("INSERT INTO accounts (account_id, balance, created_at, updated_at) VALUES ($1, $2, now(), now())")
            .bind(user_id)
            .bind(Decimal::new(10000, 2))
            .execute(pool)
            .await
            .unwrap();

        user_id
    }

    async fn insert_asset(pool: &PgPool, account_id: Uuid, asset_type: &str) -> Uuid {
        let asset_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO assets (id, account_id, asset_type, balance, created_at, updated_at)
                     VALUES ($1, $2, $3, $4, now(), now())",
        )
        .bind(asset_id)
        .bind(account_id)
        .bind(asset_type)
        .bind(Decimal::new(5000, 2))
        .execute(pool)
        .await
        .unwrap();
        asset_id
    }

    #[tokio::test]
    async fn test_create_update_delete_transaction() {
        let pool = setup_test_db().await;

        let from_account_id = insert_user_and_account(&pool).await;
        let to_account_id = insert_user_and_account(&pool).await;

        let from_asset_id = insert_asset(&pool, from_account_id, "cash").await;
        let to_asset_id = insert_asset(&pool, to_account_id, "bank").await;

        // Create
        let tx = create_transaction(
            &pool,
            Some(from_asset_id),
            Some(to_asset_id),
            TransactionType::InternalTransfer,
            Decimal::new(1500, 2),
            Some(Decimal::new(50, 2)),
            Some(from_account_id),
            Some(to_account_id),
            Some(Utc::now()),
            Some("Test transfer".to_string()),
            None,
        )
        .await
        .expect("Transaction creation failed");

        assert_eq!(tx.amount, Decimal::new(1500, 2));
        assert_eq!(tx.fee, Decimal::new(50, 2));
        assert_eq!(tx.notes.as_deref(), Some("Test transfer"));

        // Update
        let updated = update_transaction_info(
            &pool,
            tx.id,
            None,
            None,
            Some(TransactionType::Expense),
            Some(Decimal::new(1000, 2)),
            None,
            None,
            None,
            Some(Utc::now()),
            Some("updated note".to_string()),
            Some("image.jpg".to_string()),
        )
        .await
        .expect("Update failed");

        assert_eq!(updated.transaction_type, TransactionType::Expense);
        assert_eq!(updated.notes.as_deref(), Some("updated note"));

        // Get by ID
        let fetched = get_transaction_by_transation_id(&pool, tx.id)
            .await
            .expect("Fetch by ID failed");
        assert_eq!(fetched.id, tx.id);

        // Get by account
        let enriched = get_transactions_by_account_id(&pool, from_account_id)
            .await
            .expect("Get by account failed");
        assert!(enriched.iter().any(|etx| etx.id == tx.id));
        assert_eq!(enriched[0].from_asset_type.as_deref(), Some("cash"));
        assert_eq!(enriched[0].to_asset_type.as_deref(), Some("bank"));

        // Delete
        delete_transaction(&pool, tx.id)
            .await
            .expect("Delete failed");
        let result = get_transaction_by_transation_id(&pool, tx.id).await;
        assert!(matches!(result, Err(sqlx::Error::RowNotFound)));
    }
}
