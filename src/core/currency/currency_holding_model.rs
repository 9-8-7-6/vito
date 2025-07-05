use axum::response::{IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

/// Represents a currency holding record from the database
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct CurrencyHolding {
    /// Unique identifier for the holding record
    pub id: Uuid,

    /// Associated account ID
    pub account_id: Uuid,

    /// Country code (e.g., "US", "JP")
    pub country: String,

    /// Currency code (e.g., "USD", "JPY")
    pub currency_code: String,

    /// Amount of currency held
    pub amount_held: Decimal,

    /// Average cost price in TWD per unit of foreign currency
    pub average_cost_per_unit: Option<Decimal>,

    /// Record creation timestamp
    pub created_at: DateTime<Utc>,

    /// Record last update timestamp
    pub updated_at: DateTime<Utc>,
}

/// Allows a `CurrencyHolding` instance to be returned directly as a JSON response in Axum route handlers
impl IntoResponse for CurrencyHolding {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

/// Wrapper for a list of currency holdings used when returning multiple records
#[derive(Debug, Serialize)]
pub struct CurrencyHoldingList(pub Vec<CurrencyHolding>);

/// Allows `CurrencyHoldingList` to be returned directly as a JSON response in Axum route handlers
impl IntoResponse for CurrencyHoldingList {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
