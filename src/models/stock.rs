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
    pub ticker_symble: Uuid,
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

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct StockInfo {
    pub country: String,
    pub ticker_symbol: String,
    pub company_name: String,
    pub trade_volume: String,
    pub trade_value: String,
    pub opening_price: String,
    pub highest_price: String,
    pub lowest_price: String,
    pub closing_price: String,
    pub change: String,
    pub transaction: String,
}

impl IntoResponse for StockInfo {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Serialize)]
pub struct StockInfoList(pub Vec<StockInfo>);

impl IntoResponse for StockInfoList {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
