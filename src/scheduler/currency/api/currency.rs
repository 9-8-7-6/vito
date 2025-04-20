use crate::models::Currency;
use reqwest::Client;
use serde::Deserialize;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

/// Represents the API response structure from MetalPriceAPI
#[derive(Debug, Deserialize)]
struct ApiResponse {
    /// Whether the API call was successful
    success: bool,

    /// The base currency (should be "TWD" in this case)
    base: String,

    /// Timestamp of the response
    timestamp: u64,

    /// A map of exchange rates: key = currency code, value = rate
    rates: HashMap<String, f64>,
}

/// Represents one record from the local CSV that maps currency codes to their names
#[derive(Debug, Deserialize)]
struct CsvCurrency {
    #[serde(rename = "Code")]
    code: String,

    #[serde(rename = "Name")]
    name: String,
}

/// Loads the currency name mapping from the CSV file into a HashMap
///
/// # Returns
/// A map of currency codes to currency names (e.g., "USD" => "United States Dollar")
fn load_currency_name_map() -> HashMap<String, String> {
    let mut rdr = csv::Reader::from_path("src/scheduler/currency/api/metalpriceapi_currencies.csv")
        .expect("failed to read CSV");
    let mut map = HashMap::new();
    for result in rdr.deserialize::<CsvCurrency>() {
        if let Ok(row) = result {
            map.insert(row.code.clone(), row.name.clone());
        }
    }
    map
}

/// Fetches TWD-based currency exchange rates from MetalPriceAPI
/// and maps them to full currency names from a local CSV file.
///
/// Only rates whose keys start with `"TWD"` (like `"TWDUSD"`, `"TWDEUR"`) will be processed.
///
/// # Returns
/// A list of `Currency` structs (each containing id, code, name, rate)
pub async fn fetch_twd_currency_rates() -> Result<Vec<Currency>, Box<dyn std::error::Error>> {
    // MetalPriceAPI endpoint with TWD as base
    let url = "https://api.metalpriceapi.com/v1/latest?api_key=be68f8ebc0fe0f6e9582d3be42cffe11&base=TWD";

    // Fetch the response body
    let response = reqwest::get(url).await?;
    let body = response.text().await?;

    // Parse JSON into ApiResponse struct
    let parsed: ApiResponse = serde_json::from_str(&body)?;

    // Load local currency name mappings
    let name_map = load_currency_name_map();
    let mut currencies: Vec<Currency> = Vec::new();

    // Only process entries with keys like TWDUSD, TWDEUR, etc.
    for (code, rate) in parsed.rates.iter() {
        if code.starts_with("TWD") && code != "TWD" {
            let stripped = code.trim_start_matches("TWD");
            let name = name_map
                .get(stripped)
                .cloned()
                .unwrap_or_else(|| "Unknown".to_string());

            currencies.push(Currency {
                id: Uuid::new_v4(),
                code: stripped.to_string(),
                name,
                rate: Some(rate.to_string()),
            });
        }
    }

    Ok(currencies)
}
