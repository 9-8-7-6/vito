use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::{PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::models::{EnrichedTransaction, Transaction, TransactionType};
use crate::repository::asset_repository::get_asset_type_by_asset_id;

const QUERY_SELECT_BY_ACCOUNT_ID: &str =
    "SELECT * FROM transactions WHERE from_account_id = $1 OR to_account_id = $1";
const QUERY_SELECT_BY_TRANSACTION_ID: &str = "SELECT * FROM transactions WHERE id = $1";
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
const QUERY_DELETE: &str = "DELETE FROM transactions WHERE id = $1";

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
        let from_asset_type = match tx.from_asset_id {
            Some(asset_id) => Some(get_asset_type_by_asset_id(pool, asset_id).await?),
            None => None,
        };

        let to_asset_type = match tx.to_asset_id {
            Some(asset_id) => Some(get_asset_type_by_asset_id(pool, asset_id).await?),
            None => None,
        };

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

pub async fn get_transaction_by_transation_id(
    pool: &PgPool,
    transaction_id: Uuid,
) -> Result<Transaction, sqlx::Error> {
    sqlx::query_as::<_, Transaction>(QUERY_SELECT_BY_TRANSACTION_ID)
        .bind(transaction_id)
        .fetch_one(pool)
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
    match sqlx::query_as::<_, Transaction>(QUERY_INSERT)
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

    if let Some(from_asset_id) = from_asset_id {
        builder.push("from_asset_id = ").push_bind(from_asset_id);
        builder.push(", ");
    }

    if let Some(to_asset_id) = to_asset_id {
        builder.push("to_asset_id = ").push_bind(to_asset_id);
        builder.push(", ");
    }

    if let Some(transaction_type) = transaction_type {
        builder
            .push("transaction_type = ")
            .push_bind(transaction_type);
        builder.push(", ");
    }

    if let Some(amount) = amount {
        builder.push("amount = ").push_bind(amount);
        builder.push(", ");
    }

    if let Some(fee) = fee {
        builder.push("fee = ").push_bind(fee);
        builder.push(", ");
    }

    if let Some(from_account_id) = from_account_id {
        builder
            .push("from_account_id = ")
            .push_bind(from_account_id);
        builder.push(", ");
    }

    if let Some(to_account_id) = to_account_id {
        builder.push("to_account_id = ").push_bind(to_account_id);
        builder.push(", ");
    }

    if let Some(transaction_time) = transaction_time {
        builder
            .push("transaction_time = ")
            .push_bind(transaction_time);
        builder.push(", ");
    }

    if let Some(notes) = notes {
        builder.push("notes = ").push_bind(notes);
        builder.push(", ");
    }

    if let Some(image) = image {
        builder.push("image = ").push_bind(image);
        builder.push(", ");
    }
    builder.push("updated_at = ").push_bind(Utc::now());

    builder.push(" WHERE id = ").push_bind(transaction_id);
    builder.push(" RETURNING *");

    let query = builder.build_query_as::<Transaction>();
    let transaction = query.fetch_one(pool).await?;

    Ok(transaction)
}

pub async fn delete_transaction(pool: &PgPool, transaction_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(QUERY_DELETE)
        .bind(transaction_id)
        .execute(pool)
        .await?;

    Ok(())
}
