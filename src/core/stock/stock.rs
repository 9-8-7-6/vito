use axum::response::{IntoResponse, Json};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Represents a user's stock holding record stored in the database.
/// This is the raw structure corresponding to the `stock_holdings` table.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct StockHolding {
    /// Unique ID of the stock holding
    pub id: Uuid,

    /// ID of the associated user (foreign key to `accounts`)
    pub account_id: Uuid,

    /// ID of the related stock (foreign key to `stock_metadata`)
    pub stock_id: Uuid,

    /// Number of shares held by the user
    pub quantity: Decimal,

    /// Average purchase price per share
    pub average_price: Decimal,

    /// Timestamp when this record was created
    pub created_at: DateTime<Utc>,

    /// Timestamp when this record was last modified
    pub updated_at: DateTime<Utc>,
}

/// Represents a stock holding enriched with metadata and latest price info.
/// This is the structure returned to the frontend UI.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct StockHoldingResponse {
    pub id: Uuid,
    pub account_id: Uuid,
    pub stock_id: Uuid,

    /// Name of the company (from stock metadata)
    pub company_name: String,

    /// Quantity of shares held
    pub quantity: Decimal,

    /// Average purchase price
    pub average_price: Decimal,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    /// Ticker symbol (e.g. AAPL, 2330)
    pub ticker_symbol: String,

    /// Most recent price (retrieved from market API)
    pub current_price: String,
}

/// Enables StockHolding to be returned directly as a JSON response in Axum
impl IntoResponse for StockHolding {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

/// Wrapper for returning a list of enriched stock holdings
#[derive(Debug, Serialize)]
pub struct StockHoldingList(pub Vec<StockHoldingResponse>);

impl IntoResponse for StockHoldingList {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

/// Represents static metadata about a stock (e.g., name, symbol, country)
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct StockMetadata {
    pub id: Uuid,

    /// Country of the exchange the stock belongs to (e.g., US, TW)
    pub country: String,

    /// Ticker symbol (e.g., AAPL, 2330)
    pub ticker_symbol: String,

    /// Name of the company
    pub name: String,
}

impl IntoResponse for StockMetadata {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

/// Wrapper for returning multiple stock metadata entries
#[derive(Debug, Serialize)]
pub struct StockMetadataList(pub Vec<StockMetadata>);

impl IntoResponse for StockMetadataList {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

/// Represents real-time or latest stock market data fetched from external APIs
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct StockInfo {
    /// Country of the exchange the stock is listed on (e.g., "US", "TW")
    pub country: String,

    /// Stock ticker symbol (e.g., AAPL, MSFT, 2330)
    pub ticker_symbol: String,

    /// Full name of the company (e.g., Apple Inc.)
    pub company_name: String,

    /// Total trading volume for the day (in shares)
    pub trade_volume: String,

    /// Total trading value (in currency, e.g., NT$, USD)
    pub trade_value: String,

    /// Opening price (first traded price of the day)
    pub opening_price: String,

    /// Highest price reached during the day
    pub highest_price: String,

    /// Lowest price during the day
    pub lowest_price: String,

    /// Last closing price (end of day)
    pub closing_price: String,

    /// Price change from previous close (absolute or %)
    pub change: String,

    /// Number of transactions (executed orders)
    pub transaction: String,
}

impl IntoResponse for StockInfo {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

/// Wrapper for returning multiple stock info entries
#[derive(Debug, Serialize)]
pub struct StockInfoList(pub Vec<StockInfo>);

impl IntoResponse for StockInfoList {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
