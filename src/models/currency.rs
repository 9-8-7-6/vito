use axum::response::{IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Represents a currency record from the database
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Currency {
    /// Unique identifier for the currency
    pub id: Uuid,

    /// Currency code (e.g., "USD", "EUR", "TWD")
    pub code: String,

    /// Full currency name (e.g., "United States Dollar", "Euro", "New Taiwan Dollar")
    pub name: String,

    /// Optional unit (e.g., "Troy Ounce" for precious metals, "1" for fiat)
    pub unit: Option<String>,
}

/// Allows a `Currency` instance to be returned directly as a JSON response in Axum route handlers
impl IntoResponse for Currency {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

/// Wrapper for a list of currencies used when returning multiple records
#[derive(Debug, Serialize)]
pub struct CurrencyList(pub Vec<Currency>);

/// Allows `CurrencyList` to be returned directly as a JSON response in Axum route handlers
impl IntoResponse for CurrencyList {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
