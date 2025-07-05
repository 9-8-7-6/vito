use sqlx::{PgPool, Row, QueryBuilder, Postgres};
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::Utc;

use super::currency_holding_model::CurrencyHolding;

/// SQL query: Get all currency holdings for an account
const QUERY_SELECT_BY_ACCOUNT_ID: &str = r#"
    SELECT * FROM currency_holding
    WHERE account_id = $1
"#;

/// SQL query: Insert or update currency holding (average price recalculated)
const QUERY_INSERT_OR_UPDATE: &str = "
    INSERT INTO currency_holding (id, account_id, country, currency_code, amount_held, average_cost_per_unit, created_at, updated_at)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
    ON CONFLICT (account_id, currency_code)
    DO UPDATE SET
        amount_held = currency_holding.amount_held + EXCLUDED.amount_held,
        average_cost_per_unit = (
            (currency_holding.amount_held * COALESCE(currency_holding.average_cost_per_unit, 0)) + 
            (EXCLUDED.amount_held * EXCLUDED.average_cost_per_unit)
        ) / (currency_holding.amount_held + EXCLUDED.amount_held),
        updated_at = EXCLUDED.updated_at
    RETURNING *;
";

/// SQL query: Delete a currency holding by ID
const QUERY_DELETE: &str = "DELETE FROM currency_holding WHERE id = $1";

/// Get all holdings for an account
pub async fn get_currency_holdings_by_account_id(
    pool: &PgPool,
    account_id: Uuid,
) -> Result<Vec<CurrencyHolding>, sqlx::Error> {
    sqlx::query_as::<_, CurrencyHolding>(QUERY_SELECT_BY_ACCOUNT_ID)
        .bind(account_id)
        .fetch_all(pool)
        .await
}

/// Create or update a currency holding (insert or upsert logic)
pub async fn create_currency_holding(
    pool: &PgPool,
    account_id: Uuid,
    country: &String,
    currency_code: &String,
    amount_held: Decimal,
    average_cost_per_unit: Decimal,
) -> Result<CurrencyHolding, sqlx::Error> {
    sqlx::query_as::<_, CurrencyHolding>(QUERY_INSERT_OR_UPDATE)
        .bind(Uuid::new_v4())
        .bind(account_id)
        .bind(country)
        .bind(currency_code)
        .bind(amount_held)
        .bind(Some(average_cost_per_unit))
        .bind(Utc::now())
        .bind(Utc::now())
        .fetch_one(pool)
        .await
}

/// Update a currency holding's amount_held and/or average price
pub async fn update_currency_holding_info(
    pool: &PgPool,
    currency_holding_id: Uuid,
    amount_held: Option<Decimal>,
    average_cost_per_unit: Option<Decimal>,
) -> Result<CurrencyHolding, sqlx::Error> {
    if amount_held.is_none() && average_cost_per_unit.is_none() {
        return Err(sqlx::Error::RowNotFound);
    }

    let mut builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE currency_holding SET ");

    if let Some(amount_held) = amount_held {
        builder.push("amount_held = ").push_bind(amount_held).push(", ");
    }

    if let Some(average_cost_per_unit) = average_cost_per_unit {
        builder.push("average_cost_per_unit = ").push_bind(Some(average_cost_per_unit)).push(", ");
    }

    builder.push("updated_at = ").push_bind(Utc::now());
    builder.push(" WHERE id = ").push_bind(currency_holding_id);
    builder.push(" RETURNING *");

    let query = builder.build_query_as::<CurrencyHolding>();
    query.fetch_one(pool).await
}

/// Delete a currency holding by ID
pub async fn delete_currency_holding(
    pool: &PgPool,
    currency_holding_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query(QUERY_DELETE)
        .bind(currency_holding_id)
        .execute(pool)
        .await
        .map(|_| ())
}
