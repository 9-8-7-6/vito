use axum::response::{IntoResponse, Json};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Defines the type of a transaction (e.g. income, expense, transfer)
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[repr(i32)] // Stored as integers in the database
pub enum TransactionType {
    /// Represents incoming funds
    Income = 1,
    /// Represents outgoing funds
    Expense = 2,
    /// Represents a transfer between two accounts
    Transfer = 3,
    /// Internal movement of funds within the same account (e.g., rebalancing)
    InternalTransfer = 4,
}

/// Represents a financial transaction, including transfers, incomes, and expenses
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    /// Unique transaction ID
    pub id: Uuid,

    /// Asset the funds are coming from (nullable for income)
    pub from_asset_id: Option<Uuid>,

    /// Asset the funds are going to (nullable for expense)
    pub to_asset_id: Option<Uuid>,

    /// Type of transaction
    pub transaction_type: TransactionType,

    /// Main amount of the transaction
    pub amount: Decimal,

    /// Any additional fee associated with the transaction
    pub fee: Decimal,

    /// Account ID where the funds are coming from (nullable)
    pub from_account_id: Option<Uuid>,

    /// Account ID where the funds are going to (nullable)
    pub to_account_id: Option<Uuid>,

    /// Timestamp when the transaction was created in the system
    pub created_at: DateTime<Utc>,

    /// Timestamp when the transaction was last updated
    pub updated_at: DateTime<Utc>,

    /// Timestamp representing the actual transaction time (optional, user-defined)
    pub transaction_time: Option<DateTime<Utc>>,

    /// Optional notes attached to the transaction
    pub notes: Option<String>,

    /// Optional image URL or base64 string associated with the transaction
    pub image: Option<String>,
}

/// Allows the `Transaction` struct to be returned as a JSON response
impl IntoResponse for Transaction {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

/// Default values for a new transaction (mainly used for initialization or testing)
impl Default for Transaction {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            from_asset_id: None,
            to_asset_id: None,
            transaction_type: TransactionType::Expense,
            amount: Decimal::ZERO,
            fee: Decimal::ZERO,
            from_account_id: None,
            to_account_id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            transaction_time: None,
            notes: None,
            image: None,
        }
    }
}

/// Wrapper for returning a list of transactions
#[derive(Debug, Serialize)]
pub struct TransactionList(pub Vec<Transaction>);

/// Enables `TransactionList` to be returned as a JSON response
impl IntoResponse for TransactionList {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

/// Extended version of `Transaction` used for frontend APIs,
/// includes `from_asset_type` and `to_asset_type` for easier display
#[derive(Debug, Serialize)]
pub struct EnrichedTransaction {
    pub id: Uuid,
    pub from_asset_id: Option<Uuid>,
    pub to_asset_id: Option<Uuid>,
    pub transaction_type: TransactionType,
    pub amount: Decimal,
    pub fee: Decimal,
    pub from_account_id: Option<Uuid>,
    pub to_account_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub transaction_time: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub image: Option<String>,

    /// Human-readable asset type of the source asset (e.g., "cash", "bank")
    pub from_asset_type: Option<String>,

    /// Human-readable asset type of the destination asset
    pub to_asset_type: Option<String>,
}

/// Wrapper for returning a list of enriched transactions with asset type details
#[derive(Debug, Serialize)]
pub struct EnrichedTransactionList(pub Vec<EnrichedTransaction>);

/// Enables `EnrichedTransactionList` to be returned as a JSON response
impl IntoResponse for EnrichedTransactionList {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
