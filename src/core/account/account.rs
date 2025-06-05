use axum::response::{IntoResponse, Json};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Represents an account entity stored in the database
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Account {
    /// Unique identifier for the account
    pub account_id: Uuid,

    /// The current balance of the account
    pub balance: Decimal,

    /// Timestamp indicating when the account was created
    pub created_at: DateTime<Utc>,

    /// Timestamp indicating when the account was last updated
    pub updated_at: DateTime<Utc>,
}

/// Allows the Account struct to be returned directly in Axum route handlers as a JSON response
impl IntoResponse for Account {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

/// Wrapper type for returning a list of accounts
#[derive(Debug, Serialize)]
pub struct AccountList(pub Vec<Account>);

/// Allows AccountList to be returned directly in Axum route handlers as a JSON response
impl IntoResponse for AccountList {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
