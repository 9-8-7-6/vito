use axum::response::{IntoResponse, Json};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

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

#[derive(Debug, Serialize, Clone)]
pub struct CurrencyHoldingResponse {
    pub id: Uuid,
    pub account_id: Uuid,
    pub country: String,
    pub currency_code: String,
    pub amount_held: Decimal,
    pub average_cost_per_unit: Option<Decimal>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub current_price: Option<Decimal>,
}

#[derive(Debug, Serialize)]
pub struct CurrencyHoldingResponseList(pub Vec<CurrencyHoldingResponse>);

impl IntoResponse for CurrencyHoldingResponseList {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
