use super::super::api::stock_metadata::fetch_stock_metadata_by_country;
use crate::repository::create_or_update_stock_metadata;

use chrono::Utc;
use cron::Schedule;
use sqlx::PgPool;
use std::{str::FromStr, time::Duration};
use tokio::time::sleep;

pub async fn update_stock_metadata_every_month(
    pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = run_stock_metadata_job(pool).await {
        eprintln!("Initial stock metadata update failed: {}", e);
    }

    // Schedule: Every month on the 1st at 00:00
    let expression = "0 0 0 1 * * *";
    let schedule = Schedule::from_str(expression)?;

    loop {
        if let Some(next) = schedule.upcoming(Utc).next() {
            let now = Utc::now();
            let duration_secs = (next - now).num_seconds().max(0) as u64;

            println!("Next stock info update scheduled at: {}", next);

            sleep(Duration::from_secs(duration_secs)).await;

            if let Err(e) = run_stock_metadata_job(pool).await {
                eprintln!("Monthly stock metadata update failed: {}", e);
            }
        }
    }
}

async fn run_stock_metadata_job(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let datas = fetch_stock_metadata_by_country("TW").await?;

    create_or_update_stock_metadata(pool, datas).await?;
    println!("Fetching stock metadata successfully");
    Ok(())
}
