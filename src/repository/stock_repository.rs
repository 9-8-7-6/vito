use crate::scheduler::stock::api::stock_metadata::Metadata;
use chrono::Utc;
use rust_decimal::Decimal;
use sqlx::{PgPool, Postgres, QueryBuilder, Row};
use uuid::Uuid;

use crate::models::{StockHolding, StockHoldingResponse, StockInfo, StockMetadata};

/// ===============================
/// STOCK HOLDINGS
/// ===============================

/// SQL query: Join stock holdings with metadata and market price data
const QUERY_SELECT_BY_ACCOUNT_ID: &str = r#"
    SELECT 
        stock_holdings.*, 
        stock_metadata.ticker_symbol,
        stock_infos.company_name,
        stock_infos.closing_price AS current_price
    FROM stock_holdings
    JOIN stock_metadata 
        ON stock_metadata.id = stock_holdings.stock_id
    JOIN stock_infos 
        ON stock_infos.ticker_symbol = stock_metadata.ticker_symbol
    WHERE stock_holdings.account_id = $1
"#;

/// SQL query: Get stock ID by country and ticker symbol
const QUERY_STOCK_ID_FROM_STOCK_METADATA: &str =
    "SELECT id FROM stock_metadata WHERE country = $1 AND ticker_symbol = $2 AND is_active = TRUE";

/// SQL query: Insert or update stock holding (average price recalculated)
const QUERY_INSERT_OR_UPDATE: &str = "
    INSERT INTO stock_holdings (id, account_id, stock_id, quantity, average_price, created_at, updated_at)
    VALUES ($1, $2, $3, $4, $5, $6, $7)
    ON CONFLICT (account_id, stock_id)
    DO UPDATE SET
        quantity = stock_holdings.quantity + EXCLUDED.quantity,
        average_price = (
            (stock_holdings.quantity * stock_holdings.average_price) + 
            (EXCLUDED.quantity * EXCLUDED.average_price)
        ) / (stock_holdings.quantity + EXCLUDED.quantity),
        updated_at = EXCLUDED.updated_at
    RETURNING *;
";

/// SQL query: Delete a stock holding by ID
const QUERY_DELETE: &str = "DELETE FROM stock_holdings WHERE id = $1";

/// Get all holdings for an account, including metadata and market price
pub async fn get_stock_holdings_by_account_id(
    pool: &PgPool,
    account_id: Uuid,
) -> Result<Vec<StockHoldingResponse>, sqlx::Error> {
    sqlx::query_as::<_, StockHoldingResponse>(QUERY_SELECT_BY_ACCOUNT_ID)
        .bind(account_id)
        .fetch_all(pool)
        .await
}

/// Create or update a stock holding (insert or upsert logic)
pub async fn create_stock_holding(
    pool: &PgPool,
    account_id: Uuid,
    country: String,
    ticker_symbol: &String,
    quantity: Decimal,
    average_price: Decimal,
) -> Result<StockHolding, sqlx::Error> {
    let stock_id = sqlx::query(QUERY_STOCK_ID_FROM_STOCK_METADATA)
        .bind(country)
        .bind(ticker_symbol)
        .fetch_optional(pool)
        .await?
        .map(|row| row.get::<Uuid, _>("id"))
        .ok_or(sqlx::Error::RowNotFound)?;

    sqlx::query_as::<_, StockHolding>(QUERY_INSERT_OR_UPDATE)
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

/// Update a stock holding's quantity and/or average price
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
        builder.push("quantity = ").push_bind(quantity).push(", ");
    }

    if let Some(average_price) = average_price {
        builder
            .push("average_price = ")
            .push_bind(average_price)
            .push(", ");
    }

    builder.push("updated_at = ").push_bind(Utc::now());
    builder.push(" WHERE id = ").push_bind(stock_holding_id);
    builder.push(" RETURNING *");

    let query = builder.build_query_as::<StockHolding>();
    query.fetch_one(pool).await
}

/// Delete a stock holding by ID
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

/// ===============================
/// STOCK METADATA
/// ===============================

const QUERY_METADATA_SELECT_ALL: &str = "SELECT * FROM stock_metadata";
const QUERY_METADATA_SELECT_BY_ID: &str = "SELECT * FROM stock_metadata WHERE id = $1";
const QUERY_METADATA_UPSERT: &str = "
    INSERT INTO stock_metadata (id, country, ticker_symbol, name, is_active)
    VALUES ($1, $2, $3, $4, TRUE)
    ON CONFLICT (country, ticker_symbol)
    DO UPDATE SET 
        name = EXCLUDED.name,
        is_active = TRUE
";
const QUERY_METADATA_DELETE_ALL: &str = "DELETE FROM stock_metadata";
const QUERY_METADATA_DELETE: &str = "DELETE FROM stock_metadata WHERE id = $1";

/// Get all stock metadata entries
pub async fn get_all_stock_metadata(pool: &PgPool) -> Result<Vec<StockMetadata>, sqlx::Error> {
    sqlx::query_as::<_, StockMetadata>(QUERY_METADATA_SELECT_ALL)
        .fetch_all(pool)
        .await
}

/// Get a single stock metadata record by ID
pub async fn get_stock_metadata_by_id(
    pool: &PgPool,
    id: Uuid,
) -> Result<StockMetadata, sqlx::Error> {
    sqlx::query_as::<_, StockMetadata>(QUERY_METADATA_SELECT_BY_ID)
        .bind(id)
        .fetch_one(pool)
        .await
}

/// Insert or update multiple stock metadata records
pub async fn create_or_update_stock_metadata(
    pool: &PgPool,
    datas: Vec<Metadata>,
) -> Result<(), sqlx::Error> {
    for data in datas {
        sqlx::query(QUERY_METADATA_UPSERT)
            .bind(Uuid::new_v4())
            .bind(data.country)
            .bind(data.ticker_symbol)
            .bind(data.company_name)
            .execute(pool)
            .await?;
    }
    Ok(())
}

/// Update selected fields of a stock metadata record
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
    query.fetch_one(pool).await
}

/// Delete a single metadata record by ID
pub async fn delete_stock_metadata(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(QUERY_METADATA_DELETE)
        .bind(id)
        .execute(pool)
        .await
        .map(|_| ())
}

/// Delete all metadata entries
pub async fn delete_all_stock_metadata(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(QUERY_METADATA_DELETE_ALL)
        .execute(pool)
        .await
        .map(|_| ())
}

/// ===============================
/// STOCK INFOS (Market Data)
/// ===============================

/// SQL query: Insert or update real-time market data
const QUERY_UPSERT_STOCK_INFO: &str = "
    INSERT INTO stock_infos (
        country, ticker_symbol, company_name, trade_volume,
        trade_value, opening_price, highest_price, lowest_price,
        closing_price, \"change\", transaction
    )
    VALUES (
        $1, $2, $3, $4,
        $5, $6, $7, $8,
        $9, $10, $11
    )
    ON CONFLICT (country, ticker_symbol)
    DO UPDATE SET 
        company_name = EXCLUDED.company_name,
        trade_volume = EXCLUDED.trade_volume,
        trade_value = EXCLUDED.trade_value,
        opening_price = EXCLUDED.opening_price,
        highest_price = EXCLUDED.highest_price,
        lowest_price = EXCLUDED.lowest_price,
        closing_price = EXCLUDED.closing_price,
        change = EXCLUDED.change,
        transaction = EXCLUDED.transaction
";

/// Insert or update a single stock info record
pub async fn create_or_insert_stock_info(
    pool: &PgPool,
    info: StockInfo,
) -> Result<(), sqlx::Error> {
    sqlx::query(QUERY_UPSERT_STOCK_INFO)
        .bind(info.country)
        .bind(info.ticker_symbol)
        .bind(info.company_name)
        .bind(info.trade_volume)
        .bind(info.trade_value)
        .bind(info.opening_price)
        .bind(info.highest_price)
        .bind(info.lowest_price)
        .bind(info.closing_price)
        .bind(info.change)
        .bind(info.transaction)
        .execute(pool)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{migrate::MigrateDatabase, PgPool, Postgres};
    use std::env;

    async fn setup_test_db() -> PgPool {
        dotenvy::from_filename(".env.test").ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env.test");

        if !Postgres::database_exists(&db_url).await.unwrap_or(false) {
            Postgres::create_database(&db_url)
                .await
                .expect("Failed to create test DB");
        }

        let pool = PgPool::connect(&db_url).await.expect("Connect failed");
        sqlx::migrate!().run(&pool).await.expect("Migration failed");
        pool
    }

    #[tokio::test]
    async fn test_upsert_and_get_stock_metadata() {
        let pool = setup_test_db().await;

        let metadata = vec![Metadata {
            country: "TW".to_string(),
            ticker_symbol: "2330".to_string(),
            company_name: "台積電".to_string(),
        }];
        sqlx::query("DELETE FROM stock_metadata")
            .execute(&pool)
            .await
            .unwrap();
        create_or_update_stock_metadata(&pool, metadata.clone())
            .await
            .unwrap();
        let all = get_all_stock_metadata(&pool).await.unwrap();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].ticker_symbol, "2330");

        // update name test
        create_or_update_stock_metadata(
            &pool,
            vec![Metadata {
                country: "TW".to_string(),
                ticker_symbol: "2330".to_string(),
                company_name: "TSMC".to_string(),
            }],
        )
        .await
        .unwrap();

        let updated = get_all_stock_metadata(&pool).await.unwrap();
        assert_eq!(updated[0].name, "TSMC");
    }

    #[tokio::test]
    async fn test_insert_stock_info_and_query_holding() {
        let pool = setup_test_db().await;

        // setup metadata first
        let metadata = Metadata {
            country: "TW".to_string(),
            ticker_symbol: "2330".to_string(),
            company_name: "TSMC".to_string(),
        };
        create_or_update_stock_metadata(&pool, vec![metadata])
            .await
            .unwrap();

        // insert stock info
        let info = StockInfo {
            country: "TW".to_string(),
            ticker_symbol: "2330".to_string(),
            company_name: "TSMC".to_string(),
            trade_volume: "10000".to_string(),
            trade_value: "100000".to_string(),
            opening_price: "50.0".to_string(),
            highest_price: "51.0".to_string(),
            lowest_price: "49.0".to_string(),
            closing_price: "50.5".to_string(),
            change: "0.5".to_string(),
            transaction: "500".to_string(),
        };
        create_or_insert_stock_info(&pool, info).await.unwrap();
    }
}
