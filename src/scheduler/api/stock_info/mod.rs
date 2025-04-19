// Submodule for fetching TWSE (Taiwan Stock Exchange) stock info
pub mod tw;

use crate::models::StockInfo;
use tw::call_stock_info_api as call_tw;

/// Fetches stock information for a given country
///
/// # Arguments
/// * `country` - Country code (e.g., "TW" for Taiwan)
///
/// # Returns
/// * `Ok(Vec<StockInfo>)` if data is successfully fetched
/// * `Err(...)` if the country is unsupported or API call fails
pub async fn fetch_stock_info_by_country(
    country: &str,
) -> Result<Vec<StockInfo>, Box<dyn std::error::Error>> {
    match country {
        "TW" => call_tw().await, // Call TWSE-specific API implementation
        _ => Err("Unsupported country".into()), // Extend this match block for other countries
    }
}
