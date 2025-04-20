use crate::scheduler::stock::api::stock_metadata::common::Metadata;
use reqwest::Client;
use serde::Deserialize;

/// Represents the structure of the US stock metadata response from Finnhub
#[derive(Debug, Deserialize)]
pub struct StockApiResponse {
    #[serde(rename = "symbol")]
    ticker_symbol: String,

    #[serde(rename = "description")]
    company_name: String,

    #[serde(rename = "type")]
    type_: String,
}

/// Fetches US stock metadata (ticker symbol + company name) from the Finnhub API.
///
/// # Returns
/// - `Ok(Vec<Metadata>)`: A normalized list of stock metadata
/// - `Err(...)`: On network or deserialization failure
pub async fn call_us_metadata_api(
) -> Result<Vec<Metadata>, Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();

    let skip_types = [
        "MLP",
        "Ltd Part",
        "NVDR",
        "Stapled Security",
        "Savings Share",
        "Dutch Cert",
        "NY Reg Shrs",
        "PRIVATE",
        "Misc.",
        "",
    ];

    // Send a GET request to Finnhub's stock symbol endpoint for the US exchange
    let response = client
        .get("https://finnhub.io/api/v1/stock/symbol?exchange=US&token=d003ee1r01qud9qlh5bgd003ee1r01qud9qlh5c0")
        .send()
        .await?;

    // Convert the HTTP response body to a string
    let text = response.text().await?;

    // Attempt to deserialize JSON into a vector of StockApiResponse structs
    let parsed = serde_json::from_str::<Vec<StockApiResponse>>(&text);

    let json_data = match parsed {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to parse US stock metadata JSON: {}", e);
            return Err(Box::new(e));
        }
    };

    // Convert parsed API results into internal Metadata model
    // Filter out entries that are missing symbol or name
    let result = json_data
        .into_iter()
        .filter(|d| {
            !d.ticker_symbol.is_empty()
                && !d.company_name.is_empty()
                && !skip_types.contains(&d.type_.as_str())
        })
        .map(|data| Metadata {
            country: "US".to_string(),
            ticker_symbol: data.ticker_symbol,
            company_name: data.company_name,
        })
        .collect();

    Ok(result)
}
