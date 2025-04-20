use axum::{extract::State, http::StatusCode, response::IntoResponse};
use sqlx::PgPool;
use std::sync::Arc;

use crate::repository::fetch_all_countries;

/// Handler: Fetch all available countries from the database
/// Returns a list of `Country` records as a JSON response.
/// On failure, responds with 500 Internal Server Error.
pub async fn get_all_countries_handler(State(pool): State<Arc<PgPool>>) -> impl IntoResponse {
    match fetch_all_countries(&pool).await {
        Ok(countries) => countries.into_response(),
        Err(err) => {
            eprintln!("Failed to fetch all countries: {:#?}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
