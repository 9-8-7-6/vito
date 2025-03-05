use crate::models::Asset;
use crate::repository::{
    create_asset, delete_asset, get_asset_by_id, get_assets, update_asset_info,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use rust_decimal::Decimal;
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub async fn get_all_assets(
    State(pool): State<Arc<PgPool>>,
) -> Result<Json<Vec<Asset>>, StatusCode> {
    match get_assets(&pool).await {
        Ok(assets) => Ok(Json(assets)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_asset(
    State(pool): State<Arc<PgPool>>,
    Path(asset_id): Path<Uuid>,
) -> Result<Json<Asset>, StatusCode> {
    match get_asset_by_id(&pool, asset_id).await {
        Ok(asset) => Ok(Json(asset)),
        Err(_) => Err(StatusCode::NOT_FOUND),
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
) -> Result<(StatusCode, Json<Asset>), StatusCode> {
    match create_asset(
        &pool,
        payload.account_id,
        payload.asset_type,
        payload.balance,
    )
    .await
    {
        Ok(asset) => Ok((StatusCode::CREATED, Json(asset))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
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
) -> Result<(StatusCode, Json<Asset>), StatusCode> {
    match update_asset_info(&pool, asset_id, payload.asset_type, payload.balance).await {
        Ok(asset) => Ok((StatusCode::OK, Json(asset))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_asset_handler(
    State(pool): State<Arc<PgPool>>,
    Path(asset_id): Path<Uuid>,
) -> StatusCode {
    match delete_asset(&pool, asset_id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
