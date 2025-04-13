use axum::{routing::get, Router};
use axum_login::login_required;
use sqlx::PgPool;
use std::sync::Arc;

use crate::handlers::country_handler::*;
use crate::models::Backend;

pub fn country_routes(state: Arc<PgPool>) -> Router {
    Router::new()
        .route("/countries", get(get_all_countries_handler))
        .with_state(state)
}
