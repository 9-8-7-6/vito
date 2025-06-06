use axum::response::{IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Represents a country record from the database
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Country {
    /// Unique identifier for the country
    pub id: Uuid,

    /// Country code (e.g., "US", "TW")
    pub code: String,

    /// Full country name (e.g., "United States", "Taiwan")
    pub name: String,

    /// Optional region name (e.g., "Americas", "Asia")
    pub region: Option<String>,

    /// Optional subregion name (e.g., "Northern America", "Eastern Asia")
    pub subregion: Option<String>,

    /// Optional list of time zones associated with the country
    pub timezone: Option<Vec<String>>,

    /// Optional URL to the country's flag image
    pub flag_url: Option<String>,
}

/// Allows a `Country` instance to be returned directly as a JSON response in Axum route handlers
impl IntoResponse for Country {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

/// Wrapper for a list of countries used when returning multiple records
#[derive(Debug, Serialize)]
pub struct CountryList(pub Vec<Country>);

/// Allows `CountryList` to be returned directly as a JSON response in Axum route handlers
impl IntoResponse for CountryList {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
