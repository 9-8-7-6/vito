pub mod tw;

use tw::call_stock_metadata_api as call_tw;

pub mod common;
pub use common::Metadata;

pub async fn fetch_stock_metadata_by_country(
    country: &str,
) -> Result<Vec<Metadata>, Box<dyn std::error::Error>> {
    match country {
        "TW" => call_tw().await,
        _ => Err("Unsupported country".into()),
    }
}
