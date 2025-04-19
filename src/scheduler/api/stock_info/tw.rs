use crate::models::StockInfo;
use crate::repository::create_or_insert_stock_info;
use reqwest::Client;
use serde::Deserialize;
use sqlx::PgPool;

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

/// Calls the TWSE API to fetch daily stock info for all listed companies in Taiwan,
/// parses the response, and stores or updates each record into the database.
///
/// # Arguments
/// * `pool` - Shared database connection pool
///
/// # Returns
/// * `Ok(())` if all records are inserted or updated successfully
/// * `Err(...)` if network, deserialization, or DB error occurs
pub async fn call_twse_info_api(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize an HTTP client
    let client = Client::new();

    // Call the TWSE open API to retrieve all listed stock daily data
    let response = client
        .get("https://openapi.twse.com.tw/v1/exchangeReport/STOCK_DAY_ALL")
        .send()
        .await?;

    // Read response body as plain text
    let text = response.text().await?;

    // Deserialize JSON into a vector of intermediate structs
    let json_data: Vec<StockApiResponse> = serde_json::from_str(&text)?;

    // Convert each API response entry to StockInfo and insert into DB
    for data in json_data {
        let info = StockInfo {
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
        };

        // Insert or update stock info record
        create_or_insert_stock_info(pool, info).await?;
    }

    Ok(())
}
