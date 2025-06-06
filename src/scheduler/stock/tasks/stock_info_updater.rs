use super::super::api::stock_info::fetch_stock_info_by_country;

use chrono::Utc;
use cron::Schedule;
use sqlx::PgPool;
use std::{str::FromStr, time::Duration};
use tokio::time::sleep;

/// Starts a background scheduler that fetches and stores stock market data daily.
///
/// - Runs immediately on startup
/// - Repeats every day at 00:00 UTC using cron-based timing
///
/// # Arguments
/// * `pool` - Shared database connection pool
pub async fn update_stock_info_every_day(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    // Run the job immediately after startup
    if let Err(e) = run_stock_info_job(pool).await {
        eprintln!("Initial stock info update failed: {}", e);
    }

    // Cron expression: "0 0 0 * * *" => every day at 00:00
    let expression = "0 0 0 * * *";
    let schedule = Schedule::from_str(expression)?;

    loop {
        // Get the next scheduled execution time
        if let Some(next) = schedule.upcoming(Utc).next() {
            let now = Utc::now();
            let duration_secs = (next - now).num_seconds().max(0) as u64;

            println!("Next stock info update scheduled at: {}", next);

            // Sleep until the next scheduled time
            sleep(Duration::from_secs(duration_secs)).await;

            // Execute the fetch-and-save job
            if let Err(e) = run_stock_info_job(pool).await {
                eprintln!("Scheduled stock info update failed: {}", e);
            }
        }
    }
}

/// Performs the actual data update job:
/// - Fetches real-time/daily stock data for TW and US
/// - Stores them into the database via `fetch_stock_info_by_country`
async fn run_stock_info_job(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    for country in ["TW"] {
        fetch_stock_info_by_country(pool, country).await?;
    }

    println!("Fetched and updated stock info successfully.");
    Ok(())
}
