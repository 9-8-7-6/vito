use axum::{routing::get, Router};
use axum_login::login_required;
use sqlx::PgPool;
use std::sync::Arc;

use crate::handlers::country_handler::*;
use crate::models::Backend;

/// Defines routes related to country data
pub fn country_routes(state: Arc<PgPool>) -> Router {
    Router::new()
        // GET /countries -> Fetch all countries from the database
        .route("/countries", get(get_all_countries_handler))
        // Share database connection pool (PgPool) with handler functions
        .with_state(state)
}
