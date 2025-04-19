// Submodule for fetching TWSE (Taiwan Stock Exchange) stock info
pub mod tw;
pub mod us;

use sqlx::PgPool;
use tw::call_twse_info_api;
use us::call_us_se_info_api;

/// Fetches stock information for a given country
///
/// # Arguments
/// * `country` - Country code (e.g., "TW" for Taiwan)
///
/// # Returns
/// * `Ok(Vec<StockInfo>)` if data is successfully fetched
/// * `Err(...)` if the country is unsupported or API call fails
pub async fn fetch_stock_info_by_country(
    pool: &PgPool,
    country: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match country {
        "TW" => call_twse_info_api(pool).await,
        "US" => call_us_se_info_api(pool).await,
        _ => Err("Unsupported country".into()),
    }
}
