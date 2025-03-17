use axum::response::{IntoResponse, Json};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "INTEGER")]
pub enum TransactionType {
    Income = 1,
    Expense = 2,
    Transfer = 3,
    InternalTransfer = 4,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Transaction {
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
}

impl IntoResponse for Transaction {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

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

#[derive(Debug, Serialize)]
pub struct TransactionList(pub Vec<Transaction>);

impl IntoResponse for TransactionList {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
