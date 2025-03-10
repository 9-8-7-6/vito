use crate::handlers::login_logout_handler::*;
use crate::models::Backend;
use axum::{routing::post, Router};

pub fn login_routes(backend: Backend) -> Router {
    Router::new()
        .route("/api/login", post(api_login))
        .route("/api/logout", post(api_logout))
        .with_state(backend)
}
