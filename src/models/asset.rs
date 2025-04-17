use axum::response::{IntoResponse, Json};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Represents an asset belonging to an account
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Asset {
    /// Unique identifier for the asset
    pub id: Uuid,

    /// The ID of the account this asset belongs to
    pub account_id: Uuid,

    /// Type of asset (e.g., "cash", "stock", etc.)
    pub asset_type: String,

    /// The current balance of this asset
    pub balance: Decimal,

    /// Timestamp indicating when the asset was created
    pub created_at: DateTime<Utc>,

    /// Timestamp indicating when the asset was last updated
    pub updated_at: DateTime<Utc>,
}

/// Allows an Asset instance to be returned directly as a JSON HTTP response
impl IntoResponse for Asset {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

/// Wrapper struct for returning a list of assets
#[derive(Debug, Serialize)]
pub struct AssetList(pub Vec<Asset>);

/// Allows an AssetList to be returned directly as a JSON HTTP response
impl IntoResponse for AssetList {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
