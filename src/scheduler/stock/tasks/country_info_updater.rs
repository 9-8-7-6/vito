use super::super::api::country_info::call_country_info_api;
use crate::repository::upsert_country;

use chrono::Utc;
use cron::Schedule;
use sqlx::PgPool;
use std::{str::FromStr, time::Duration};
use tokio::time::sleep;

/// Starts a background task that fetches and updates country metadata once per month.
///
/// The task is triggered immediately at startup, then scheduled to run on the 1st of every month at midnight.
pub async fn update_country_info_every_month(
    pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Run the job immediately once at startup
    if let Err(e) = run_country_info_job(pool).await {
        eprintln!("Initial country info update failed: {}", e);
    }

    // Cron expression: At 00:00 on the 1st day of each month
    let expression = "0 0 0 1 * * *";
    let schedule = Schedule::from_str(expression)?;

    loop {
        // Calculate delay until next scheduled run
        if let Some(next) = schedule.upcoming(Utc).next() {
            let now = Utc::now();
            let duration_secs = (next - now).num_seconds().max(0) as u64;

            println!("Next country info update scheduled at: {}", next);

            // Sleep until the scheduled time
            sleep(Duration::from_secs(duration_secs)).await;

            // Run the job
            if let Err(e) = run_country_info_job(pool).await {
                eprintln!("Scheduled country info update failed: {}", e);
            }
        }
    }
}

/// Executes the actual job: fetches country data from the API and persists it to the database
async fn run_country_info_job(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    // Fetch country metadata from external API
    let datas = call_country_info_api().await?;

    // Insert or update each country in the database
    upsert_country(pool, datas).await?;

    println!("Fetching country info successfully.");
    Ok(())
}
