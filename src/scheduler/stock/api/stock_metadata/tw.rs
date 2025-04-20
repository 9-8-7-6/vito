use crate::scheduler::stock::api::stock_metadata::common::Metadata;
use reqwest::Client;
use serde::Deserialize;

/// Represents the structure of the TWSE company metadata API response
#[derive(Debug, Deserialize)]
struct StockApiResponse {
    #[serde(rename = "公司代號")]
    ticker_symbol: String,

    #[serde(rename = "公司簡稱")]
    company_name: String,
}

/// Fetches stock metadata (ticker + company name) from the Taiwan Stock Exchange (TWSE).
///
/// # Returns
/// - `Ok(Vec<Metadata>)`: Parsed and normalized stock metadata
/// - `Err(...)`: On network failure or deserialization error
pub async fn call_twse_metadata_api(
) -> Result<Vec<Metadata>, Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();

    // Send GET request to TWSE open data API
    let response = client
        .get("https://openapi.twse.com.tw/v1/opendata/t187ap03_L")
        .send()
        .await?;

    // Read the response body as text
    let text = response.text().await?;

    // Attempt to deserialize JSON into a vector of `StockApiResponse`
    let parsed = serde_json::from_str::<Vec<StockApiResponse>>(&text);

    let json_data = match parsed {
        Ok(data) => data,
        Err(e) => {
            // Log deserialization failure for debugging
            eprintln!("Failed to parse TW stock metadata JSON: {}", e);
            return Err(Box::new(e));
        }
    };

    // Convert parsed API results into internal Metadata model
    let result = json_data
        .into_iter()
        .map(|data| Metadata {
            country: "TW".to_string(),
            ticker_symbol: data.ticker_symbol,
            company_name: data.company_name,
        })
        .collect();

    Ok(result)
}
