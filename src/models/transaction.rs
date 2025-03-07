use axum::response::{IntoResponse, Json};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::convert::TryFrom;
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[repr(i16)]
#[sqlx(type_name = "INTEGER")]
pub enum TransactionType {
    Income = 1,
    Expense = 2,
    Transfer = 3,
    InternalTransfer = 4,
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TransactionType::Income => "Income",
            TransactionType::Expense => "Expense",
            TransactionType::Transfer => "Transfer",
            TransactionType::InternalTransfer => "Internal Transfer",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub id: Uuid,
    pub from_asset_id: Option<Uuid>,
    pub to_asset_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
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
}

impl IntoResponse for Transaction {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Serialize)]
pub struct TransactionList(pub Vec<Transaction>);

impl IntoResponse for TransactionList {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
