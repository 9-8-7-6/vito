use reqwest::Client;
use serde::Deserialize;
use crate::models::StockInfo;

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

pub async fn call_stock_info_api() -> Result<Vec<StockInfo>, Box<dyn std::error::Error>> {
    let client = Client::new();

    let response = client
        .get("https://openapi.twse.com.tw/v1/exchangeReport/STOCK_DAY_ALL")
        .send()
        .await?;
    let text = response.text().await?;

    let json_data: Vec<StockApiResponse> = serde_json::from_str(&text)?;

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
