use axum::{routing::post, Router};
use axum_login::login_required;

use crate::handlers::login_logout_handler::*;
use crate::models::Backend;

pub fn login_routes(backend: Backend) -> Router {
    Router::new()
        .route("/api/logout", post(api_logout))
        .route("/api/delete_account", post(api_delete_account))
        .route_layer(login_required!(Backend, login_url = "/login"))
        .route("/api/login", post(api_login))
        .route("/api/register", post(api_register))
        .with_state(backend)
}
