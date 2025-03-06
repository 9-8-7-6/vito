use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum IntervalChoices {
    Daily,
    Weekly,
    Monthly,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "INTEGER")]
pub enum TransactionType {
    Income = 1,
    Expense = 2,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RecurringTransaction {
    pub id: Uuid,
    pub account_id: Uuid,
    pub asset_id: Uuid,
    pub category_id: Option<Uuid>,
    pub amount: Decimal,
    pub interval: IntervalChoices,
    pub next_execution: DateTime<Utc>,
    pub transaction_type: TransactionType,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
