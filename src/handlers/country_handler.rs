use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use rust_decimal::Decimal;
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::models::Country;
use crate::repository::fetch_all_countries;

pub async fn get_all_countries_handler(State(pool): State<Arc<PgPool>>) -> impl IntoResponse {
    match fetch_all_countries(&pool).await {
        Ok(countries) => countries.into_response(),
        Err(err) => {
            eprintln!("Failed to fetch all countries: {:#?}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
