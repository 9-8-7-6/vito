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

use crate::models::AssetList;
use crate::repository::{
    create_asset, delete_asset, get_asset_by_user_id, get_assets, update_asset_info,
};

/// Request payload for creating a new asset
#[derive(Deserialize)]
pub struct CreateAssetRequest {
    pub account_id: Uuid,
    pub asset_type: String,
    pub balance: Decimal,
}

/// Request payload for updating an existing asset
#[derive(Deserialize)]
pub struct UpdateAssetRequest {
    pub asset_type: Option<String>,
    pub balance: Option<Decimal>,
}

/// Handler: Fetch all asset records from the system
pub async fn get_all_assets_handler(State(pool): State<Arc<PgPool>>) -> impl IntoResponse {
    match get_assets(&pool).await {
        Ok(assets) => AssetList(assets).into_response(),
        Err(err) => {
            eprintln!("Failed to fetch all assets: {:#?}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Handler: Fetch all assets for a specific user (by account ID)
pub async fn get_asset_handler(
    State(pool): State<Arc<PgPool>>,
    Path(user_id): Path<Uuid>,
) -> impl IntoResponse {
    match get_asset_by_user_id(&pool, user_id).await {
        Ok(assets) => AssetList(assets).into_response(),
        Err(err) => {
            eprintln!("Failed to fetch assets for user {}: {:#?}", user_id, err);
            StatusCode::NOT_FOUND.into_response()
        }
    }
}

/// Handler: Create a new asset for a user account
pub async fn add_asset_handler(
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
        Err(err) => {
            eprintln!(
                "Failed to create asset for account {}: {:#?}",
                payload.account_id, err
            );
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Handler: Update an asset’s type or balance
pub async fn update_asset_handler(
    State(pool): State<Arc<PgPool>>,
    Path(asset_id): Path<Uuid>,
    Json(payload): Json<UpdateAssetRequest>,
) -> impl IntoResponse {
    match update_asset_info(&pool, asset_id, payload.asset_type, payload.balance).await {
        Ok(asset) => asset.into_response(),
        Err(err) => {
            eprintln!("Failed to update asset {}: {:#?}", asset_id, err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Handler: Delete an asset by its ID
pub async fn delete_asset_handler(
    State(pool): State<Arc<PgPool>>,
    Path(asset_id): Path<Uuid>,
) -> impl IntoResponse {
    match delete_asset(&pool, asset_id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(), // 204 No Content
        Err(err) => {
            eprintln!("Failed to delete asset {}: {:#?}", asset_id, err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
