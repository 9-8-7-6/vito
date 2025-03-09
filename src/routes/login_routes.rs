use crate::handlers::login_handler::*;
use axum::{routing::post, Router};
use sqlx::PgPool;
use std::sync::Arc;

pub fn login_routes(state: Arc<PgPool>) -> Router {
    Router::new()
        .route("/api/login", post(api_login))
        .with_state(state)
}
