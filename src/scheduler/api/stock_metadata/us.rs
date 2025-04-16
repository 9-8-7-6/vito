use crate::scheduler::api::stock_metadata::common::Metadata;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct StockApiResponse {
    #[serde(rename = "symbol")]
    ticker_symbol: String,

    #[serde(rename = "description")]
    company_name: String,
}

pub async fn call_us_metadata_api(
) -> Result<Vec<Metadata>, Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();

    let response = client
        .get("https://finnhub.io/api/v1/stock/symbol?exchange=US&token=d003ee1r01qud9qlh5bgd003ee1r01qud9qlh5c0")
        .send()
        .await?;
    let text = response.text().await?;

    let parsed = serde_json::from_str::<Vec<StockApiResponse>>(&text);

    let json_data = match parsed {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to parse US stock metadata JSON : {}", e);
            return Err(Box::new(e));
        }
    };

    let result = json_data
        .into_iter()
        .filter(|d| !d.ticker_symbol.is_empty() && !d.company_name.is_empty())
        .map(|data| Metadata {
            country: "US".to_string(),
            ticker_symbol: data.ticker_symbol,
            company_name: data.company_name,
        })
        .collect();

    Ok(result)
}
