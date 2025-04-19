// TWSE (Taiwan) stock metadata API implementation
pub mod tw;
// US stock metadata API implementation (e.g., via Finnhub)
pub mod us;

// Re-export the shared metadata model
pub mod common;
pub use common::Metadata;

use tw::call_twse_metadata_api;
use us::call_us_metadata_api;

/// Fetches stock metadata for a specific country by delegating to the appropriate API caller.
///
/// # Arguments
/// * `country` - ISO country code (e.g., "TW" or "US")
///
/// # Returns
/// * A list of `Metadata` structs representing stock ticker and company info
/// * An error if the country is unsupported or the API call fails
pub async fn fetch_stock_metadata_by_country(
    country: &str,
) -> Result<Vec<Metadata>, Box<dyn std::error::Error + Send + Sync>> {
    match country {
        "TW" => call_twse_metadata_api().await, // Fetch from TWSE
        "US" => call_us_metadata_api().await,   // Fetch from US API (e.g., Finnhub)
        _ => Err("Unsupported country".into()), // Return error for unsupported markets
    }
}
