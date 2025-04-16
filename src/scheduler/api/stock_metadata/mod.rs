pub mod tw;

use tw::call_twse_metadata_api;

pub mod common;
pub use common::Metadata;

pub async fn fetch_stock_metadata_by_country(
    country: &str,
) -> Result<Vec<Metadata>, Box<dyn std::error::Error>> {
    match country {
        "TW" => call_twse_metadata_api().await,
        _ => Err("Unsupported country".into()),
    }
}
