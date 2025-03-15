use chrono::Utc;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::Asset;

const QUERY_SELECT_ALL: &str = "SELECT * FROM assets";
const QUERY_SELECT_ONE: &str = "SELECT * FROM assets WHERE id = $1";
const QUERY_INSERT: &str = "
    INSERT INTO assets (id, account_id, asset_type, balance, created_at, updated_at) 
    VALUES ($1, $2, $3, $4, $5, $6) 
    RETURNING *
";
const QUERY_UPDATE: &str = "
    UPDATE assets
    SET asset_type = $1, balance = $2, updated_at = $3
    WHERE id = $4
    RETURNING *
";
const QUERY_DELETE: &str = "DELETE FROM assets WHERE id = $1";

pub async fn get_assets(pool: &PgPool) -> Result<Vec<Asset>, sqlx::Error> {
    sqlx::query_as::<_, Asset>(QUERY_SELECT_ALL)
        .fetch_all(pool)
        .await
}

pub async fn get_asset_by_user_id(pool: &PgPool, asset_id: Uuid) -> Result<Asset, sqlx::Error> {
    sqlx::query_as::<_, Asset>(QUERY_SELECT_ONE)
        .bind(asset_id)
        .fetch_one(pool)
        .await
}

pub async fn create_asset(
    pool: &PgPool,
    account_id: Uuid,
    asset_type: String,
    balance: Decimal,
) -> Result<Asset, sqlx::Error> {
    sqlx::query_as::<_, Asset>(QUERY_INSERT)
        .bind(Uuid::new_v4())
        .bind(account_id)
        .bind(asset_type)
        .bind(balance)
        .bind(Utc::now())
        .bind(Utc::now())
        .fetch_one(pool)
        .await
}

pub async fn update_asset_info(
    pool: &PgPool,
    asset_id: Uuid,
    asset_type: String,
    balance: Decimal,
) -> Result<Asset, sqlx::Error> {
    sqlx::query_as::<_, Asset>(QUERY_UPDATE)
        .bind(asset_type)
        .bind(balance)
        .bind(Utc::now())
        .bind(asset_id)
        .fetch_one(pool)
        .await
}

pub async fn delete_asset(pool: &PgPool, asset_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(QUERY_DELETE)
        .bind(asset_id)
        .execute(pool)
        .await
        .map(|_| ())
}
