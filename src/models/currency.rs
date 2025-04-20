use axum::response::{IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Represents a single currency record stored in the database.
/// Each record contains a unique ID, a standardized currency code (e.g. USD, EUR),
/// the full name of the currency, and an optional exchange rate value.
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Currency {
    /// Universally unique identifier for the currency record.
    pub id: Uuid,

    /// Standardized currency code (e.g., "USD" for US Dollar, "TWD" for New Taiwan Dollar).
    pub code: String,

    /// Descriptive name of the currency (e.g., "United States Dollar", "Euro").
    pub name: String,

    /// Optional exchange rate relative to a base currency (e.g., 32.5 for USD to TWD),
    /// or unit reference (e.g., "Troy Ounce" for metals).
    pub rate: Option<String>,
}

/// Enables a `Currency` struct to be returned directly in HTTP responses
/// as a JSON payload in Axum-based handlers.
impl IntoResponse for Currency {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

/// A wrapper type for returning a list of `Currency` objects
/// as a single JSON array in an HTTP response.
#[derive(Debug, Serialize)]
pub struct CurrencyList(pub Vec<Currency>);

/// Enables `CurrencyList` to be returned as a JSON response from an Axum handler.
impl IntoResponse for CurrencyList {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
