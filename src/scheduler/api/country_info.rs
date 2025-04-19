use crate::models::Country;
use reqwest::Client;
use serde::Deserialize;
use uuid::Uuid;

/// Represents the structure of each country in the response from the REST Countries API
#[derive(Debug, Deserialize)]
struct CountryApiResponse {
    cca2: String,                   // 2-letter ISO country code
    name: CountryName,              // Country name structure
    region: Option<String>,         // Continent or region (e.g., Asia, Europe)
    subregion: Option<String>,      // Subregion (e.g., South-Eastern Asia)
    timezones: Option<Vec<String>>, // List of associated timezones
    flags: Option<FlagUrl>,         // Country flag URLs
}

/// Represents the "name" field inside the country object
#[derive(Debug, Deserialize)]
struct CountryName {
    common: String, // Common name of the country
}

/// Represents the flag URL object
#[derive(Debug, Deserialize)]
struct FlagUrl {
    png: Option<String>, // PNG-format flag URL
}

/// Calls the REST Countries API to retrieve detailed metadata about all countries.
///
/// # Returns
/// - `Ok(Vec<Country>)` containing parsed and transformed country data
/// - `Err(...)` in case of request failure or JSON deserialization errors
pub async fn call_country_info_api() -> Result<Vec<Country>, Box<dyn std::error::Error>> {
    let client = Client::new();

    // Send a GET request to the REST Countries API
    let response = client
        .get("https://restcountries.com/v3.1/all")
        .send()
        .await?;

    // Get the response body as a string
    let text = response.text().await?;

    // Deserialize the raw JSON into a vector of our intermediate structs
    let json_data: Vec<CountryApiResponse> = serde_json::from_str(&text)?;

    // Transform the API data into internal `Country` model used by the app
    let result = json_data
        .into_iter()
        .filter_map(|data| {
            Some(Country {
                id: Uuid::new_v4(),
                code: data.cca2,
                name: data.name.common,
                region: data.region,
                subregion: data.subregion,
                timezone: data.timezones,
                flag_url: data.flags.and_then(|f| f.png),
            })
        })
        .collect();

    Ok(result)
}
