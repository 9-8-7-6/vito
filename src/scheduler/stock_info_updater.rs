use crate::api::stock::call_stock_info_api;
use crate::repository::{delete_all_stock_infos, insert_stock_infos};

use chrono::Utc;
use cron::Schedule;
use sqlx::PgPool;
use std::{str::FromStr, time::Duration};
use tokio::time::sleep;

pub async fn update_stock_info_every_day(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = run_stock_info_job(pool).await {
        eprintln!("Initial stock info update failed: {}", e);
    }

    let expression = "0 0 0 1 * * *";
    let schedule = Schedule::from_str(expression)?;

    loop {
        if let Some(next) = schedule.upcoming(Utc).next() {
            let now = Utc::now();
            let duration_secs = (next - now).num_seconds().max(0) as u64;

            println!("Next stock info update scheduled at: {}", next);

            sleep(Duration::from_secs(duration_secs)).await;

            if let Err(e) = run_stock_info_job(pool).await {
                eprintln!("Scheduled stock info update failed: {}", e);
            }
        }
    }
}

async fn run_stock_info_job(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let datas = call_stock_info_api().await?;

    delete_all_stock_infos(pool).await?;
    insert_stock_infos(pool, datas).await?;

    println!("Fetching stock info successfully.");
    Ok(())
}
