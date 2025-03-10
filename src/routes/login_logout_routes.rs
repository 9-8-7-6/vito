use crate::handlers::login_logout_handler::*;
use axum::{routing::post, Router};
use sqlx::PgPool;
use std::sync::Arc;

pub fn login_routes(state: Arc<PgPool>) -> Router {
    Router::new()
        .route("/api/login", post(api_login))
        .route("/api/logout", post(api_logout))
        .with_state(state)
}
