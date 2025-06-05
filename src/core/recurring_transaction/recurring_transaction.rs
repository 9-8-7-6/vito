use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fmt;
use uuid::Uuid;

/// Defines the interval at which a recurring transaction is executed
#[derive(Debug, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")] // Maps to a TEXT column in the database
pub enum IntervalChoices {
    /// Transaction recurs daily
    Daily,
    /// Transaction recurs weekly
    Weekly,
    /// Transaction recurs monthly
    Monthly,
}

/// Allows IntervalChoices to be displayed as a string (e.g., in logs or error messages)
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

/// Specifies whether a recurring transaction is income or expense
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[repr(i32)] // Stores the enum as an integer in the database
pub enum RecurringTransactionType {
    /// Incoming funds (e.g., salary)
    Income = 1,
    /// Outgoing funds (e.g., subscription fee)
    Expense = 2,
}

/// Represents a recurring financial transaction tied to an account and asset
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RecurringTransaction {
    /// Unique ID of the recurring transaction
    pub id: Uuid,

    /// ID of the account associated with the transaction
    pub account_id: Uuid,

    /// ID of the asset (e.g., cash, bank) associated with the transaction
    pub asset_id: Uuid,

    /// Amount of the transaction
    pub amount: Decimal,

    /// Interval at which the transaction recurs (daily, weekly, monthly)
    pub interval: IntervalChoices,

    /// The next scheduled execution datetime of the transaction
    pub next_execution: DateTime<Utc>,

    /// Whether the transaction is an income or an expense
    pub transaction_type: RecurringTransactionType,

    /// Indicates if the transaction is currently active
    pub is_active: bool,

    /// When the transaction record was created
    pub created_at: DateTime<Utc>,

    /// When the transaction record was last updated
    pub updated_at: DateTime<Utc>,
}
