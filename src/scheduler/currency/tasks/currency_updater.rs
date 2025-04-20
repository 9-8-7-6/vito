use super::super::api::fetch_twd_currency_rates;
use crate::repository::upsert_currencies;

use chrono::Utc;
use cron::Schedule;
use sqlx::PgPool;
use std::{str::FromStr, time::Duration};
use tokio::time::sleep;

/// Launches a background task to periodically update currency exchange rate information
///
/// - The task runs **once immediately** at application startup
/// - Then it repeats **daily at 00:00 midnight** using a cron expression
///
/// # Arguments
/// * `pool` - A reference to the shared PostgreSQL connection pool
///
/// # Returns
/// * `Ok(())` if the scheduler starts successfully
/// * `Err(...)` if the cron expression is invalid or a runtime error occurs
pub async fn update_currency_info_every_day(
    pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Run the update job immediately on startup
    if let Err(e) = run_currency_info_job(pool).await {
        eprintln!("Initial currency info update failed: {}", e);
    }

    // Cron expression to run the task every day at 00:00 (midnight)
    // Format: sec min hour day-of-month month day-of-week year
    let expression = "0 0 0 * * * *";
    let schedule = Schedule::from_str(expression)?;

    loop {
        // Determine when the next scheduled job should run
        if let Some(next) = schedule.upcoming(Utc).next() {
            let now = Utc::now();
            let duration_secs = (next - now).num_seconds().max(0) as u64;

            println!("Next currency info update scheduled at: {}", next);

            // Wait until the scheduled time
            sleep(Duration::from_secs(duration_secs)).await;

            // Execute the scheduled job
            if let Err(e) = run_currency_info_job(pool).await {
                eprintln!("Scheduled currency info update failed: {}", e);
            }
        }
    }
}

/// Executes the actual currency update logic:
/// 1. Fetches the latest TWD-based exchange rates from MetalPriceAPI
/// 2. Inserts or updates the data into the `currencies` table
///
/// # Arguments
/// * `pool` - A reference to the PostgreSQL connection pool
///
/// # Returns
/// * `Ok(())` on success
/// * `Err(...)` if any step fails (API, parsing, or DB)
async fn run_currency_info_job(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    // Fetch latest exchange rates (TWD to other currencies)
    let datas = fetch_twd_currency_rates().await?;

    // Insert or update the fetched data into the database
    upsert_currencies(pool, datas).await?;

    println!("Fetching currency info successfully.");
    Ok(())
}
