use crate::models::AssetList;
use crate::repository::{
    create_asset, delete_asset, get_asset_by_id, get_assets, update_asset_info,
};
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

pub async fn get_all_assets(State(pool): State<Arc<PgPool>>) -> impl IntoResponse {
    match get_assets(&pool).await {
        Ok(assets) => AssetList(assets).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_asset(
    State(pool): State<Arc<PgPool>>,
    Path(asset_id): Path<Uuid>,
) -> impl IntoResponse {
    match get_asset_by_id(&pool, asset_id).await {
        Ok(asset) => asset.into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

#[derive(Deserialize)]
pub struct CreateAssetRequest {
    pub account_id: Uuid,
    pub asset_type: String,
    pub balance: Decimal,
}

pub async fn add_asset(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<CreateAssetRequest>,
) -> impl IntoResponse {
    match create_asset(
        &pool,
        payload.account_id,
        payload.asset_type,
        payload.balance,
    )
    .await
    {
        Ok(asset) => asset.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[derive(Deserialize)]
pub struct UpdateAssetRequest {
    pub asset_type: String,
    pub balance: Decimal,
}

pub async fn update_asset(
    State(pool): State<Arc<PgPool>>,
    Path(asset_id): Path<Uuid>,
    Json(payload): Json<UpdateAssetRequest>,
) -> impl IntoResponse {
    match update_asset_info(&pool, asset_id, payload.asset_type, payload.balance).await {
        Ok(asset) => asset.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn delete_asset_handler(
    State(pool): State<Arc<PgPool>>,
    Path(asset_id): Path<Uuid>,
) -> impl IntoResponse {
    match delete_asset(&pool, asset_id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
