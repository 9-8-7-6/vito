use axum::response::{IntoResponse, Json};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct StockHolding {
    pub id: Uuid,
    pub account_id: Uuid,
    pub stock_id: Uuid,
    pub quantity: Decimal,
    pub average_price: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl IntoResponse for StockHolding {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Serialize)]
pub struct StockHoldingList(pub Vec<StockHolding>);

impl IntoResponse for StockHoldingList {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct StockMetadata {
    pub id: Uuid,
    pub country: String,
    pub ticker_symbol: String,
    pub name: String,
}

impl IntoResponse for StockMetadata {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Serialize)]
pub struct StockMetadataList(pub Vec<StockMetadata>);

impl IntoResponse for StockMetadataList {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
