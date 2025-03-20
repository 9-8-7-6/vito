use chrono::{format, Utc};
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::Asset;

const QUERY_SELECT_ALL: &str = "SELECT * FROM assets";
const QUERY_SELECT_BY_USER_ID: &str = "SELECT * FROM assets WHERE account_id = $1";
const QUERY_INSERT: &str = "
    INSERT INTO assets (id, account_id, asset_type, balance, created_at, updated_at) 
    VALUES ($1, $2, $3, $4, $5, $6) 
    RETURNING *
";
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
    let mut query = String::from("UPDATE assets SET ");
    let mut params: Vec<Box<dyn sqlx::Encode<'_, sqlx::Postgres> + Send + Sync>> = Vec::new();
    let mut cnt = 1;
    
    if let Some(asset_type) = asset_type {
        query.push_str(&format!("asset_type = ${}, ", cnt));
        params.push(Box::new(asset_type));
        cnt += 1;
    }
    if let Some(balance) = balance {
        query.push_str(&format!("balance = ${}, ", cnt));
        params.push(Box::new(balance));
        cnt += 1;
    }

    if params.is_empty() {
        return Err(sqlx::Error::RowNotFound);
    }

    // 加入 updated_at 欄位
    let now = Utc::now();
    query.push_str(&format!("updated_at = ${} ", cnt));
    params.push(Box::new(now));
    cnt += 1;

    // WHERE 條件
    query.push_str(&format!("WHERE id = ${} RETURNING *", cnt));
    params.push(Box::new(asset_id));

    let mut query = sqlx::query_as::<_, Asset>(&query);

    // 綁定參數
    for param in params {
        query = query.bind(param);
    }

    query.fetch_one(pool).await
}

pub async fn delete_asset(pool: &PgPool, asset_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(QUERY_DELETE)
        .bind(asset_id)
        .execute(pool)
        .await
        .map(|_| ())
}
