use crate::models::StockInfo;
use reqwest::Client;
use serde::Deserialize;

/// Represents the expected structure of the TWSE stock API response
#[derive(Debug, Deserialize)]
struct StockApiResponse {
    #[serde(rename = "Code")]
    ticker_symbol: String,

    #[serde(rename = "Name")]
    company_name: String,

    #[serde(rename = "TradeVolume")]
    trade_volume: String,

    #[serde(rename = "TradeValue")]
    trade_value: String,

    #[serde(rename = "OpeningPrice")]
    opening_price: String,

    #[serde(rename = "HighestPrice")]
    highest_price: String,

    #[serde(rename = "LowestPrice")]
    lowest_price: String,

    #[serde(rename = "ClosingPrice")]
    closing_price: String,

    #[serde(rename = "Change")]
    change: String,

    #[serde(rename = "Transaction")]
    transaction: String,
}

/// Calls the TWSE API to fetch daily stock info for all listed companies
///
/// # Returns
/// * A list of `StockInfo` models parsed from the API response
/// * Error if the request or deserialization fails
pub async fn call_stock_info_api() -> Result<Vec<StockInfo>, Box<dyn std::error::Error>> {
    // Create a reusable HTTP client
    let client = Client::new();

    // Send a GET request to TWSE daily stock data endpoint
    let response = client
        .get("https://openapi.twse.com.tw/v1/exchangeReport/STOCK_DAY_ALL")
        .send()
        .await?;

    // Convert the HTTP response body to a string
    let text = response.text().await?;

    // Deserialize JSON array into vector of intermediate API structs
    let json_data: Vec<StockApiResponse> = serde_json::from_str(&text)?;

    // Map raw API response into your internal StockInfo model
    let result = json_data
        .into_iter()
        .map(|data| StockInfo {
            country: "TW".to_string(),
            ticker_symbol: data.ticker_symbol,
            company_name: data.company_name,
            trade_volume: data.trade_volume,
            trade_value: data.trade_value,
            opening_price: data.opening_price,
            highest_price: data.highest_price,
            lowest_price: data.lowest_price,
            closing_price: data.closing_price,
            change: data.change,
            transaction: data.transaction,
        })
        .collect();

    Ok(result)
}
