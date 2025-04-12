use crate::models::Country;
use reqwest::Client;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct CountryApiResponse {
    cca2: String,
    name: CountryName,
    region: Option<String>,
    subregion: Option<String>,
    timezones: Option<Vec<String>>,
    flags: Option<FlagUrl>,
}

#[derive(Debug, Deserialize)]
struct CountryName {
    common: String,
}

#[derive(Debug, Deserialize)]
struct FlagUrl {
    png: Option<String>,
}

pub async fn call_country_info_api() -> Result<Vec<Country>, Box<dyn std::error::Error>> {
    let client = Client::new();

    let response = client
        .get("https://restcountries.com/v3.1/all")
        .send()
        .await?;

    let text = response.text().await?;

    let json_data: Vec<CountryApiResponse> = serde_json::from_str(&text)?;

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
