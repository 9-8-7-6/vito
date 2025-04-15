pub mod tw;

use crate::models::StockInfo;
use tw::call_stock_info_api as call_tw;

pub async fn fetch_stock_info_by_country(
    country: &str,
) -> Result<Vec<StockInfo>, Box<dyn std::error::Error>> {
    match country {
        "TW" => call_tw().await,
        _ => Err("Unsupported country".into()),
    }
}
