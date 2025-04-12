use reqwest::Client;
use serde::Deserialize;

#[derive(Debug)]
pub struct Metadata {
    pub country: String,
    pub ticker_symbol: String,
    pub company_name: String,
}

#[derive(Debug, Deserialize)]
struct StockApiResponse {
    #[serde(rename = "公司代號")]
    ticker_symbol: String,

    #[serde(rename = "公司簡稱")]
    company_name: String,
}

pub async fn call_stock_metadata_api() -> Result<Vec<Metadata>, Box<dyn std::error::Error>> {
    let client = Client::new();

    let response = client
        .get("https://openapi.twse.com.tw/v1/opendata/t187ap03_L")
        .send()
        .await?;
    let text = response.text().await?;

    let parsed = serde_json::from_str::<Vec<StockApiResponse>>(&text);

    let json_data = match parsed {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to parse stock Metadata's JSON : {}", e);
            return Err(Box::new(e));
        }
    };

    let mut result = Vec::new();

    for data in json_data {
        result.push(Metadata {
            country: "TW".to_string(),
            ticker_symbol: data.ticker_symbol,
            company_name: data.company_name,
        });
    }
    Ok(result)
}
