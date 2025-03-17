use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum IntervalChoices {
    Daily,
    Weekly,
    Monthly,
}

impl fmt::Display for IntervalChoices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            IntervalChoices::Daily => "Daily",
            IntervalChoices::Weekly => "Weekly",
            IntervalChoices::Monthly => "Monthly",
        };
        write!(f, "{}", s)
    }
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
    pub amount: Decimal,
    pub interval: IntervalChoices,
    pub next_execution: DateTime<Utc>,
    pub transaction_type: TransactionType,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
