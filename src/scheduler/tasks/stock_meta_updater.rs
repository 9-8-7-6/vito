use super::super::api::stock_metadata::fetch_stock_metadata_by_country;
use crate::repository::create_or_update_stock_metadata;

use chrono::Utc;
use cron::Schedule;
use sqlx::PgPool;
use std::{str::FromStr, time::Duration};
use tokio::time::sleep;

/// Starts a background task that updates stock metadata (symbol + company name)
/// for all supported countries on the 1st of every month at midnight.
pub async fn update_stock_metadata_every_month(
    pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Run the metadata update job once at startup
    if let Err(e) = run_stock_metadata_job(pool).await {
        eprintln!("Initial stock metadata update failed: {}", e);
    }

    // Cron schedule: At 00:00 on the 1st day of every month
    let expression = "0 0 0 1 * * *";
    let schedule = Schedule::from_str(expression)?;

    loop {
        if let Some(next) = schedule.upcoming(Utc).next() {
            let now = Utc::now();
            let duration_secs = (next - now).num_seconds().max(0) as u64;

            println!("Next stock metadata update scheduled at: {}", next);

            // Sleep until the scheduled time
            sleep(Duration::from_secs(duration_secs)).await;

            // Execute the metadata update job
            if let Err(e) = run_stock_metadata_job(pool).await {
                eprintln!("Monthly stock metadata update failed: {}", e);
            }
        }
    }
}

/// Fetches and persists stock metadata (ticker symbol and company name)
/// for each supported country (e.g., TW, US).
async fn run_stock_metadata_job(
    pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    for country in ["TW", "US"] {
        match fetch_stock_metadata_by_country(country).await {
            Ok(datas) => {
                println!("Start to fetch metadata for country: {}", country);
                create_or_update_stock_metadata(pool, datas).await?;
                println!("Fetched and updated metadata for country: {}", country);
            }
            Err(err) => {
                eprintln!("Failed to fetch metadata for country {}: {}", country, err);
            }
        }
    }
    Ok(())
}
