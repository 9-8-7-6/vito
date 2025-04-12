use axum::response::{IntoResponse, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Country {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub region: Option<String>,
    pub subregion: Option<String>,
    pub timezone: Option<Vec<String>>,
    pub flag_url: Option<String>,
}

impl IntoResponse for Country {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Serialize)]
pub struct CountryList(pub Vec<Country>);

impl IntoResponse for CountryList {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
