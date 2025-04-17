use axum::response::{IntoResponse, Json};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Represents a user's stock holding record in the database
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct StockHolding {
    /// Unique ID of the holding
    pub id: Uuid,

    /// ID of the user that owns the holding
    pub account_id: Uuid,

    /// ID of the associated stock (foreign key to stock metadata)
    pub stock_id: Uuid,

    /// Quantity of shares held
    pub quantity: Decimal,

    /// Average price at which the shares were acquired
    pub average_price: Decimal,

    /// When the record was created
    pub created_at: DateTime<Utc>,

    /// When the record was last updated
    pub updated_at: DateTime<Utc>,
}

/// Enriched stock holding record returned to the frontend, includes additional fields
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct StockHoldingResponse {
    pub id: Uuid,
    pub account_id: Uuid,
    pub stock_id: Uuid,

    /// Name of the company (from stock metadata)
    pub company_name: String,

    pub quantity: Decimal,
    pub average_price: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    /// Stock symbol (e.g. "AAPL", "TSLA" in US, "2330" in TW)
    pub ticker_symbol: String,

    /// Most recent price (as string from API)
    pub current_price: String,
}

/// Allows StockHolding to be returned directly as JSON response in Axum routes
impl IntoResponse for StockHolding {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

/// Wrapper type for returning a list of stock holdings with company and price info
#[derive(Debug, Serialize)]
pub struct StockHoldingList(pub Vec<StockHoldingResponse>);

/// Enables StockHoldingList to be returned as a JSON response
impl IntoResponse for StockHoldingList {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

/// Represents static metadata about a stock (used to join with holdings)
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct StockMetadata {
    pub id: Uuid,
    pub country: String,
    pub ticker_symbol: String,

    /// Name of the company
    pub name: String,
}

/// Enables StockMetadata to be returned as a JSON response
impl IntoResponse for StockMetadata {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

/// Wrapper type for returning a list of stock metadata entries
#[derive(Debug, Serialize)]
pub struct StockMetadataList(pub Vec<StockMetadata>);

/// Enables StockMetadataList to be returned as a JSON response
impl IntoResponse for StockMetadataList {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

/// Represents real-time or latest stock market data (fetched from an API)
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct StockInfo {
    pub country: String,
    pub ticker_symbol: String,
    pub company_name: String,
    pub trade_volume: String,
    pub trade_value: String,
    pub opening_price: String,
    pub highest_price: String,
    pub lowest_price: String,
    pub closing_price: String,
    pub change: String,
    pub transaction: String,
}

/// Enables StockInfo to be returned as a JSON response
impl IntoResponse for StockInfo {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

/// Wrapper type for returning a list of stock info records (market data)
#[derive(Debug, Serialize)]
pub struct StockInfoList(pub Vec<StockInfo>);

/// Enables StockInfoList to be returned as a JSON response
impl IntoResponse for StockInfoList {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
