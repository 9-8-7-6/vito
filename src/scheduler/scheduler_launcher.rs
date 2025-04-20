use super::currency::update_currency_info_every_day;
use super::stock::tasks::{
    country_info_updater::update_country_info_every_month,
    stock_info_updater::update_stock_info_every_day,
    stock_meta_updater::update_stock_metadata_every_month,
};

use std::sync::Arc;

/// Launches all scheduled background jobs as asynchronous tasks.
///
/// This includes:
/// - Daily stock info updates (e.g., prices, volume)
/// - Monthly stock metadata refresh (e.g., symbol and company name)
/// - Monthly country info update (e.g., name, timezone, region)
///
/// Each task runs independently on its own tokio task.
pub async fn start_all_schedulers(state: Arc<sqlx::Pool<sqlx::Postgres>>) {
    // Start daily stock info updater
    let cloned_pool1 = state.clone();
    tokio::spawn(async move {
        if let Err(e) = update_stock_info_every_day(&cloned_pool1).await {
            eprintln!("update_stock_info_every_day failed: {}", e);
        }
    });

    // Start monthly stock metadata updater
    let cloned_pool2 = state.clone();
    tokio::spawn(async move {
        if let Err(e) = update_stock_metadata_every_month(&cloned_pool2).await {
            eprintln!("update_stock_metadata_every_month failed: {}", e);
        }
    });

    // Start monthly country info updater
    let cloned_pool3 = state.clone();
    tokio::spawn(async move {
        if let Err(e) = update_country_info_every_month(&cloned_pool3).await {
            eprintln!("update_country_info_every_month failed: {}", e); // <- fixed message
        }
    });

    // Start daily currency info updater
    let cloned_pool4 = state.clone();
    tokio::spawn(async move {
        if let Err(e) = update_currency_info_every_day(&cloned_pool4).await {
            eprintln!("update_currency_info_every_day failed: {}", e); // <- fixed message
        }
    });
}
