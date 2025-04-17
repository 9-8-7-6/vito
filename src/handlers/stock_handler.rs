use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use rust_decimal::Decimal;
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::models::{StockHolding, StockHoldingList, StockMetadata, StockMetadataList};
use crate::repository::{
    create_or_update_stock_metadata, create_stock_holding, delete_stock_holding,
    delete_stock_metadata, get_all_stock_metadata, get_stock_holdings_by_account_id,
    get_stock_metadata_by_id, update_stock_holding_info, update_stock_metadata,
};

/// Payload format for creating a stock holding
#[derive(Deserialize)]
pub struct CreateStockHoldingRequest {
    pub account_id: Uuid,
    pub ticker_symbol: String,
    pub quantity: Decimal,
    pub country: String,
    pub average_price: Decimal,
}

/// Payload format for updating a stock holding
#[derive(Deserialize)]
pub struct UpdateStockHoldingRequest {
    pub quantity: Option<Decimal>,
    pub average_price: Option<Decimal>,
}

/// Handler: Get all stock holdings for a specific account
pub async fn get_stock_holdings_by_account_handler(
    State(pool): State<Arc<PgPool>>,
    Path(account_id): Path<Uuid>,
) -> impl IntoResponse {
    match get_stock_holdings_by_account_id(&pool, account_id).await {
        Ok(holdings) => StockHoldingList(holdings).into_response(),
        Err(err) => {
            eprintln!(
                "Error fetching stock holdings by account {}: {:#?}",
                account_id, err
            );
            StatusCode::NOT_FOUND.into_response()
        }
    }
}

/// Handler: Create a stock holding record for an account and stock
pub async fn create_stock_holding_handler(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<CreateStockHoldingRequest>,
) -> impl IntoResponse {
    match create_stock_holding(
        &pool,
        payload.account_id,
        payload.country,
        &payload.ticker_symbol,
        payload.quantity,
        payload.average_price,
    )
    .await
    {
        Ok(holding) => holding.into_response(),
        Err(err) => {
            eprintln!(
                "Error creating stock holding for account {} and stock {}: {:#?}",
                payload.account_id, payload.ticker_symbol, err
            );
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Handler: Update quantity or average price of a stock holding
pub async fn update_stock_holding_handler(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateStockHoldingRequest>,
) -> impl IntoResponse {
    match update_stock_holding_info(&pool, id, payload.quantity, payload.average_price).await {
        Ok(holding) => holding.into_response(),
        Err(err) => {
            eprintln!("Error updating stock holding {}: {:#?}", id, err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Handler: Delete a stock holding record by its ID
pub async fn delete_stock_holding_handler(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match delete_stock_holding(&pool, id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => {
            eprintln!("Error deleting stock holding {}: {:#?}", id, err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Payload format for creating stock metadata (manual insert)
#[derive(Deserialize)]
pub struct CreateStockMetadataRequest {
    pub country: String,
    pub ticker_symbol: String,
    pub name: String,
}

/// Payload format for updating stock metadata
#[derive(Deserialize)]
pub struct UpdateStockMetadataRequest {
    pub country: Option<String>,
    pub ticker_symbol: Option<String>,
    pub name: Option<String>,
}

/// Handler: Get all stock metadata records (e.g., for admin viewing)
pub async fn get_all_stock_metadata_handler(State(pool): State<Arc<PgPool>>) -> impl IntoResponse {
    match get_all_stock_metadata(&pool).await {
        Ok(records) => StockMetadataList(records).into_response(),
        Err(err) => {
            eprintln!("Error fetching all stock metadata: {:#?}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Handler: Get a single stock metadata entry by UUID
pub async fn get_stock_metadata_by_id_handler(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match get_stock_metadata_by_id(&pool, id).await {
        Ok(record) => record.into_response(),
        Err(err) => {
            eprintln!("Error fetching stock metadata by ID {}: {:#?}", id, err);
            StatusCode::NOT_FOUND.into_response()
        }
    }
}

/// Handler: Update metadata of a stock by ID
pub async fn update_stock_metadata_handler(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateStockMetadataRequest>,
) -> impl IntoResponse {
    match update_stock_metadata(
        &pool,
        id,
        payload.country,
        payload.ticker_symbol,
        payload.name,
    )
    .await
    {
        Ok(metadata) => metadata.into_response(),
        Err(err) => {
            eprintln!("Error updating stock metadata {}: {:#?}", id, err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Handler: Delete a stock metadata entry by ID
pub async fn delete_stock_metadata_handler(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match delete_stock_metadata(&pool, id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => {
            eprintln!("Error deleting stock metadata {}: {:#?}", id, err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
