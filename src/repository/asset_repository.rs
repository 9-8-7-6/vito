use chrono::Utc;
use rust_decimal::Decimal;
use sqlx::{PgPool, Postgres, QueryBuilder, Row};
use uuid::Uuid;

use crate::models::Asset;

const QUERY_SELECT_ALL: &str = "SELECT * FROM assets";
const QUERY_SELECT_BY_USER_ID: &str = "SELECT * FROM assets WHERE account_id = $1";
const QUERY_INSERT: &str = "
    INSERT INTO assets (id, account_id, asset_type, balance, created_at, updated_at)
    VALUES ($1, $2, $3, $4, $5, $6)
    RETURNING *
";
const QUERY_UPDATE_BALANCE: &str =
    "UPDATE ASSETS SET balance = balance + $1, updated_at = now() WHERE id = $2";
const QUERY_DELETE: &str = "DELETE FROM assets WHERE id = $1";

pub async fn get_assets(pool: &PgPool) -> Result<Vec<Asset>, sqlx::Error> {
    sqlx::query_as::<_, Asset>(QUERY_SELECT_ALL)
        .fetch_all(pool)
        .await
}

pub async fn get_asset_by_user_id(pool: &PgPool, user_id: Uuid) -> Result<Vec<Asset>, sqlx::Error> {
    sqlx::query_as::<_, Asset>(QUERY_SELECT_BY_USER_ID)
        .bind(user_id)
        .fetch_all(pool)
        .await
}

pub async fn get_asset_type_by_asset_id(
    pool: &PgPool,
    asset_id: Uuid,
) -> Result<String, sqlx::Error> {
    let row = sqlx::query("SELECT asset_type FROM assets WHERE id = $1")
        .bind(asset_id)
        .fetch_one(pool)
        .await?;

    Ok(row.get("asset_type"))
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
    asset_type: Option<String>,
    balance: Option<Decimal>,
) -> Result<Asset, sqlx::Error> {
    if asset_type.is_none() && balance.is_none() {
        return Err(sqlx::Error::RowNotFound);
    }

    let mut builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE assets SET ");

    if let Some(asset_type) = asset_type {
        builder.push("asset_type = ").push_bind(asset_type);
        builder.push(", ");
    }

    if let Some(balance) = balance {
        builder.push("balance = ").push_bind(balance);
        builder.push(", ");
    }

    builder.push("updated_at = ").push_bind(Utc::now());

    builder.push(" WHERE id = ").push_bind(asset_id);
    builder.push(" RETURNING *");

    let query = builder.build_query_as::<Asset>();
    let asset = query.fetch_one(pool).await?;

    Ok(asset)
}

pub async fn update_asset_balance(
    pool: &PgPool,
    asset_id: Uuid,
    amount: Decimal,
) -> Result<Asset, sqlx::Error> {
    sqlx::query_as::<_, Asset>(QUERY_UPDATE_BALANCE)
        .bind(amount)
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
