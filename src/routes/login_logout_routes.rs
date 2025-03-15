use axum::{
    routing::{options, post},
    Router,
};
use axum_login::login_required;

use crate::handlers::login_logout_handler::*;
use crate::models::Backend;

pub fn login_routes(backend: Backend) -> Router {
    Router::new()
        .route("/logout", post(api_logout))
        .route("/logout", options(api_logout))
        .route("/delete_account", post(api_delete_account))
        // .route_layer(login_required!(Backend, login_url = "/login"))
        .route("/login", post(api_login))
        .route("/register", post(api_register))
        .route("/auth/check", post(check_session))
        .with_state(backend)
}
