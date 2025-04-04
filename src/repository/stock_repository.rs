use chrono::Utc;
use rust_decimal::Decimal;
use sqlx::{PgPool, Postgres, QueryBuilder, Row};
use uuid::Uuid;

use crate::models::{StockHolding, StockMetadata};

const QUERY_SELECT_ALL: &str = "SELECT * FROM stock_holdings";
const QUERY_SELECT_BY_ACCOUNT_ID: &str = "SELECT * FROM stock_holdings WHERE account_id = $1";
const QUERY_INSERT: &str = "
    INSERT INTO stock_holdings (id, account_id, stock_id, quantity, average_price, created_at, updated_at)
    VALUES ($1, $2, $3, $4, $5, $6, $7)
    RETURNING *
";
const QUERY_DELETE: &str = "DELETE FROM stock_holdings WHERE id = $1";

pub async fn get_stock_holdings(pool: &PgPool) -> Result<Vec<StockHolding>, sqlx::Error> {
    sqlx::query_as::<_, StockHolding>(QUERY_SELECT_ALL)
        .fetch_all(pool)
        .await
}

pub async fn get_stock_holdings_by_account_id(
    pool: &PgPool,
    account_id: Uuid,
) -> Result<Vec<StockHolding>, sqlx::Error> {
    sqlx::query_as::<_, StockHolding>(QUERY_SELECT_BY_ACCOUNT_ID)
        .bind(account_id)
        .fetch_all(pool)
        .await
}

pub async fn create_stock_holding(
    pool: &PgPool,
    account_id: Uuid,
    stock_id: Uuid,
    quantity: Decimal,
    average_price: Decimal,
) -> Result<StockHolding, sqlx::Error> {
    sqlx::query_as::<_, StockHolding>(QUERY_INSERT)
        .bind(Uuid::new_v4())
        .bind(account_id)
        .bind(stock_id)
        .bind(quantity)
        .bind(average_price)
        .bind(Utc::now())
        .bind(Utc::now())
        .fetch_one(pool)
        .await
}

pub async fn update_stock_holding_info(
    pool: &PgPool,
    stock_holding_id: Uuid,
    quantity: Option<Decimal>,
    average_price: Option<Decimal>,
) -> Result<StockHolding, sqlx::Error> {
    if quantity.is_none() && average_price.is_none() {
        return Err(sqlx::Error::RowNotFound);
    }

    let mut builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE stock_holdings SET ");

    if let Some(quantity) = quantity {
        builder.push("quantity = ").push_bind(quantity);
        builder.push(", ");
    }

    if let Some(average_price) = average_price {
        builder.push("average_price = ").push_bind(average_price);
        builder.push(", ");
    }

    builder.push("updated_at = ").push_bind(Utc::now());

    builder.push(" WHERE id = ").push_bind(stock_holding_id);
    builder.push(" RETURNING *");

    let query = builder.build_query_as::<StockHolding>();
    let asset = query.fetch_one(pool).await?;

    Ok(asset)
}

pub async fn delete_stock_holding(
    pool: &PgPool,
    stock_holding_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query(QUERY_DELETE)
        .bind(stock_holding_id)
        .execute(pool)
        .await
        .map(|_| ())
}

const QUERY_METADATA_SELECT_ALL: &str = "SELECT * FROM stock_metadata";
const QUERY_METADATA_SELECT_BY_ID: &str = "SELECT * FROM stock_metadata WHERE id = $1";
const QUERY_METADATA_INSERT: &str = "
    INSERT INTO stock_metadata (id, country, ticker_symbol, name)
    VALUES ($1, $2, $3, $4)
    RETURNING *
";
const QUERY_METADATA_DELETE: &str = "DELETE FROM stock_metadata WHERE id = $1";

pub async fn get_all_stock_metadata(pool: &PgPool) -> Result<Vec<StockMetadata>, sqlx::Error> {
    sqlx::query_as::<_, StockMetadata>(QUERY_METADATA_SELECT_ALL)
        .fetch_all(pool)
        .await
}

pub async fn get_stock_metadata_by_id(
    pool: &PgPool,
    id: Uuid,
) -> Result<StockMetadata, sqlx::Error> {
    sqlx::query_as::<_, StockMetadata>(QUERY_METADATA_SELECT_BY_ID)
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn create_stock_metadata(
    pool: &PgPool,
    country: String,
    ticker_symbol: String,
    name: String,
) -> Result<StockMetadata, sqlx::Error> {
    sqlx::query_as::<_, StockMetadata>(QUERY_METADATA_INSERT)
        .bind(Uuid::new_v4())
        .bind(country)
        .bind(ticker_symbol)
        .bind(name)
        .fetch_one(pool)
        .await
}

pub async fn update_stock_metadata(
    pool: &PgPool,
    id: Uuid,
    country: Option<String>,
    ticker_symbol: Option<String>,
    name: Option<String>,
) -> Result<StockMetadata, sqlx::Error> {
    if country.is_none() && ticker_symbol.is_none() && name.is_none() {
        return Err(sqlx::Error::RowNotFound);
    }

    let mut builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE stock_metadata SET ");
    let mut first = true;

    if let Some(c) = country {
        if !first {
            builder.push(", ");
        } else {
            first = false;
        }
        builder.push("country = ").push_bind(c);
    }

    if let Some(t) = ticker_symbol {
        if !first {
            builder.push(", ");
        } else {
            first = false;
        }
        builder.push("ticker_symbol = ").push_bind(t);
    }

    if let Some(n) = name {
        if !first {
            builder.push(", ");
        } else {
            first = false;
        }
        builder.push("name = ").push_bind(n);
    }

    builder.push(" WHERE id = ").push_bind(id);
    builder.push(" RETURNING *");

    let query = builder.build_query_as::<StockMetadata>();
    let result = query.fetch_one(pool).await?;

    Ok(result)
}

pub async fn delete_stock_metadata(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(QUERY_METADATA_DELETE)
        .bind(id)
        .execute(pool)
        .await
        .map(|_| ())
}
