use super::super::api::stock_info::fetch_stock_info_by_country;
use crate::repository::create_or_insert_stock_infos;

use chrono::Utc;
use cron::Schedule;
use sqlx::PgPool;
use std::{str::FromStr, time::Duration};
use tokio::time::sleep;

/// Starts a background task that fetches and stores daily stock trading data.
///
/// This function is executed once at startup and then runs every day at 00:00.
pub async fn update_stock_info_every_day(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    // Run the job once immediately at startup
    if let Err(e) = run_stock_info_job(pool).await {
        eprintln!("Initial stock info update failed: {}", e);
    }

    // Cron expression: Every day at 00:00
    let expression = "0 0 0 * * *";
    let schedule = Schedule::from_str(expression)?;

    loop {
        // Calculate the duration until the next scheduled run
        if let Some(next) = schedule.upcoming(Utc).next() {
            let now = Utc::now();
            let duration_secs = (next - now).num_seconds().max(0) as u64;

            println!("Next stock info update scheduled at: {}", next);

            // Sleep until the next run time
            sleep(Duration::from_secs(duration_secs)).await;

            // Run the job
            if let Err(e) = run_stock_info_job(pool).await {
                eprintln!("Scheduled stock info update failed: {}", e);
            }
        }
    }
}

/// Executes the actual job: fetches TW stock info and updates the database.
async fn run_stock_info_job(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    // Fetch Taiwan stock trading data from external API
    let datas = fetch_stock_info_by_country("TW").await?;

    // Insert or update records in the `stock_infos` table
    create_or_insert_stock_infos(pool, datas).await?;

    println!("Fetching stock info successfully.");
    Ok(())
}
