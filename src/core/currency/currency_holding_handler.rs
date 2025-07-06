use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::Deserialize;
use sqlx::PgPool;
use std::{collections::HashMap, sync::Arc};
use uuid::Uuid;

use crate::{
    core::currency::currency_holding_model::CurrencyHoldingResponse,
    scheduler::stock::api::stock_info::us,
};

use super::currency_holding_model::{
    CurrencyHolding, CurrencyHoldingList, CurrencyHoldingResponseList,
};
use super::currency_holding_repository::{
    create_currency_holding, delete_currency_holding, get_currency_holdings_by_account_id,
    update_currency_holding_info,
};
use super::currency_scraper::currency_scraper;

/// Payload format for creating a currency holding
#[derive(Deserialize)]
pub struct CreateCurrencyHoldingRequest {
    pub account_id: Uuid,
    pub country: String,
    pub currency_code: String,
    pub amount_held: Decimal,
    pub average_cost_per_unit: Decimal,
}

/// Payload format for updating a currency holding
#[derive(Deserialize)]
pub struct UpdateCurrencyHoldingRequest {
    pub amount_held: Option<Decimal>,
    pub average_cost_per_unit: Option<Decimal>,
}

/// Handler: Get all currency holdings for a specific account
pub async fn get_currency_holdings_by_account_handler(
    State(pool): State<Arc<PgPool>>,
    Path(account_id): Path<Uuid>,
) -> impl IntoResponse {
    // 2a) load the raw rows
    let holdings = match get_currency_holdings_by_account_id(&pool, account_id).await {
        Ok(h) => h,
        Err(err) => {
            eprintln!(
                "Error fetching currency holdings by account {}: {:#?}",
                account_id, err
            );
            return StatusCode::NOT_FOUND.into_response();
        }
    };

    // 2b) scrape all live rates in one go
    let codes: Vec<String> = holdings.iter().map(|h| h.currency_code.clone()).collect();
    let price_map = match currency_scraper(&codes).await {
        Ok(m) => m,
        Err(err) => {
            eprintln!("Error fetching live prices: {:#?}", err);
            HashMap::new()
        }
    };

    // 2c) zip into your new response type
    let resp: Vec<CurrencyHoldingResponse> = holdings
        .into_iter()
        .map(|h| CurrencyHoldingResponse {
            id: h.id,
            account_id: h.account_id,
            country: h.country,
            currency_code: h.currency_code.clone(),
            amount_held: h.amount_held,
            average_cost_per_unit: h.average_cost_per_unit,
            created_at: h.created_at,
            updated_at: h.updated_at,
            current_price: price_map.get(&h.currency_code).cloned(),
        })
        .collect();

    CurrencyHoldingResponseList(resp).into_response()
}

/// Handler: Create a currency holding record for an account
pub async fn create_currency_holding_handler(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<CreateCurrencyHoldingRequest>,
) -> impl IntoResponse {
    match create_currency_holding(
        &pool,
        payload.account_id,
        &payload.country,
        &payload.currency_code,
        payload.amount_held,
        payload.average_cost_per_unit,
    )
    .await
    {
        Ok(holding) => holding.into_response(),
        Err(err) => {
            eprintln!(
                "Error creating currency holding for account {} and currency {}: {:#?}",
                payload.account_id, &payload.currency_code, err
            );
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Handler: Update amount_held or average price of a currency holding
pub async fn update_currency_holding_handler(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCurrencyHoldingRequest>,
) -> impl IntoResponse {
    match update_currency_holding_info(
        &pool,
        id,
        payload.amount_held,
        payload.average_cost_per_unit,
    )
    .await
    {
        Ok(holding) => holding.into_response(),
        Err(err) => {
            eprintln!("Error updating currency holding {}: {:#?}", id, err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Handler: Delete a currency holding record by its ID
pub async fn delete_currency_holding_handler(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match delete_currency_holding(&pool, id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => {
            eprintln!("Error deleting currency holding {}: {:#?}", id, err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
