use crate::models::{asset, Asset};
use chrono::Utc;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_assets(pool: &PgPool) -> Result<Vec<Asset>, sqlx::Error> {
    let assets = sqlx::query_as::<_, Asset>("SELECT * FROM assets")
        .fetch_all(pool)
        .await?;
    Ok(assets)
}

pub async fn get_asset_by_id(pool: &PgPool, asset_id: Uuid) -> Result<Asset, sqlx::Error> {
    let asset = sqlx::query_as::<_, Asset>("SELECT * FROM assets WHERE id = $1")
        .bind(asset_id)
        .fetch_one(pool)
        .await?;
    Ok(asset)
}

pub async fn create_asset(
    pool: &PgPool,
    account_id: Uuid,
    asset_type: String,
    balance: Decimal,
) -> Result<Asset, sqlx::Error> {
    let asset = sqlx::query_as::<_, Asset>(
        "INSERT INTO assets (id, account_id, asset_type, balance, created_at, updated_at) 
        VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(account_id)
    .bind(asset_type)
    .bind(balance)
    .bind(Utc::now())
    .bind(Utc::now())
    .fetch_one(pool)
    .await?;

    Ok(asset)
}

pub async fn update_asset_info(
    pool: &PgPool,
    asset_id: Uuid,
    asset_type: String,
    balance: Decimal,
) -> Result<Asset, sqlx::Error> {
    let asset = sqlx::query_as::<_, Asset>(
        "UPDATE assets
        SET asset_type = $1, balance = $2, updated_at = $3
        WHERE id = $4 RETURNING *",
    )
    .bind(asset_type)
    .bind(balance)
    .bind(Utc::now())
    .bind(asset_id)
    .fetch_one(pool)
    .await?;

    Ok(asset)
}

pub async fn delete_asset(pool: &PgPool, asset_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM assets WHERE id = $1")
        .bind(asset_id)
        .execute(pool)
        .await?;

    Ok(())
}
