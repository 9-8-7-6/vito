use axum::{routing::get, Router};
use sqlx::PgPool;
use std::sync::Arc;

use crate::core::country::country_handler::*;

/// Defines routes related to country data
pub fn country_routes(state: Arc<PgPool>) -> Router {
    Router::new()
        // GET /countries -> Fetch all countries from the database
        .route("/countries", get(get_all_countries_handler))
        // Share database connection pool (PgPool) with handler functions
        .with_state(state)
}
