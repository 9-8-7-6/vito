use super::tasks::{
    country_info_updater::update_country_info_every_month,
    stock_info_updater::update_stock_info_every_day,
    stock_meta_updater::update_stock_metadata_every_month,
};

use std::sync::Arc;

pub async fn start_all_schedulers(state: Arc<sqlx::Pool<sqlx::Postgres>>) {
    let cloned_pool1: Arc<sqlx::Pool<sqlx::Postgres>> = state.clone();
    tokio::spawn(async move {
        if let Err(e) = update_stock_info_every_day(&cloned_pool1).await {
            eprintln!("update_stock_info_every_day failed: {}", e);
        }
    });

    let cloned_pool2 = state.clone();
    tokio::spawn(async move {
        if let Err(e) = update_stock_metadata_every_month(&cloned_pool2).await {
            eprintln!("update_stock_metadata_every_month failed: {}", e);
        }
    });

    let cloned_pool3 = state.clone();
    tokio::spawn(async move {
        if let Err(e) = update_country_info_every_month(&cloned_pool3).await {
            eprintln!("update_stock_metadata_every_month failed: {}", e);
        }
    });
}
