use crate::models::StockInfo;
use crate::repository::create_or_insert_stock_info;
use reqwest::Client;
use serde::Deserialize;
use sqlx::PgPool;
use tokio::time::{sleep, Duration};

/// Represents metadata of US stocks from Finnhub symbol API
#[derive(Debug, Deserialize)]
struct SymbolMetadata {
    /// Stock ticker (e.g., AAPL, MSFT, etc.)
    symbol: String,

    /// Full company name
    description: String,

    /// Asset type (e.g., Common Stock, ETF, MLP, etc.)
    #[serde(rename = "type")]
    type_: String,
}

/// Represents quote response structure from Finnhub /quote API
#[derive(Debug, Deserialize)]
struct QuoteResponse {
    #[serde(rename = "c")]
    current_price: f64,
    #[serde(rename = "d")]
    change: Option<f64>,
    #[serde(rename = "h")]
    high: f64,
    #[serde(rename = "l")]
    low: f64,
    #[serde(rename = "o")]
    open: f64,
}

/// Fetches US stock quote data from Finnhub API and stores it into database
///
/// # Arguments
/// * `pool` - Shared database connection pool
///
/// # Returns
/// * `Ok(())` if all valid records are inserted/updated successfully
/// * `Err(...)` if any network or DB error occurs
pub async fn call_us_se_info_api(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // Step 1: Fetch all US stock symbols
    let res = client
        .get("https://finnhub.io/api/v1/stock/symbol?exchange=US&token=d003ee1r01qud9qlh5bgd003ee1r01qud9qlh5c0")
        .send()
        .await?;

    let symbols: Vec<SymbolMetadata> = res.json().await?;

    // Skip certain asset types that are not for retail investors
    let skip_types = [
        "MLP",              // Master Limited Partnerships
        "Ltd Part",         // Limited Partnership
        "NVDR",             // Thai-specific depository receipts
        "Stapled Security", // Often non-retail structure
        "Savings Share",    // Italy-specific
        "Dutch Cert",       // Legal structure, not freely tradable
        "NY Reg Shrs",      // NY-registered shares (foreign)
        "PRIVATE",
        "Misc.",
        "",
    ];

    let mut cnt = 1;

    // Step 2: Loop through each symbol to get real-time quote
    for meta in symbols.iter() {
        println!("Fetching #{}: {}", cnt, meta.symbol);
        cnt += 1;

        if skip_types.contains(&meta.type_.as_str()) {
            continue;
        }

        let url = format!(
            "https://finnhub.io/api/v1/quote?symbol={}&token=d003ee1r01qud9qlh5bgd003ee1r01qud9qlh5c0",
            meta.symbol
        );

        let quote_res = client.get(&url).send().await?;
        let status = quote_res.status();
        let body = quote_res.text().await?; // response body as string

        if status.is_success() {
            match serde_json::from_str::<QuoteResponse>(&body) {
                Ok(quote) => {
                    let info = StockInfo {
                        country: "US".to_string(),
                        ticker_symbol: meta.symbol.clone(),
                        company_name: meta.description.clone(),
                        trade_volume: "-".to_string(), // Finnhub does not provide volume here
                        trade_value: "-".to_string(),
                        opening_price: quote.open.to_string(),
                        highest_price: quote.high.to_string(),
                        lowest_price: quote.low.to_string(),
                        closing_price: quote.current_price.to_string(),
                        change: quote.change.unwrap_or(0.0).to_string(),
                        transaction: "-".to_string(),
                    };

                    // Insert or update record into the database
                    create_or_insert_stock_info(pool, info).await?;
                }
                Err(e) => {
                    eprintln!("Failed to parse quote for {}: {}", meta.symbol, e);
                    eprintln!("Raw response: {}", body);
                }
            }
        } else {
            eprintln!("Non-success HTTP {} for symbol {}", status, meta.symbol);
        }

        // To respect rate limits (60 calls/minute), wait between requests
        sleep(Duration::from_millis(800)).await;
    }

    Ok(())
}
