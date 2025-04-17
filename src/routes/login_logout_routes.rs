use axum::{
    routing::{options, post},
    Router,
};
use axum_login::login_required;

use crate::handlers::login_logout_handler::*;
use crate::models::Backend;

/// Defines authentication and account management routes
pub fn login_routes(backend: Backend) -> Router {
    Router::new()
        // POST /logout     -> Logs the user out
        // OPTIONS /logout  -> CORS preflight request (optional but common in frontend-heavy apps)
        .route("/logout", post(api_logout))
        .route("/logout", options(api_logout))

        // POST /delete_account -> Deletes the current authenticated user's account
        .route("/delete_account", post(api_delete_account))

        // Optionally protect routes with session-based login
        // .route_layer(login_required!(Backend, login_url = "/login"))

        // POST /login     -> Authenticate user credentials and set session
        .route("/login", post(api_login))

        // POST /register  -> Create a new user account
        .route("/register", post(api_register))

        // POST /auth/check -> Verify if current session is valid (used for frontend checks)
        .route("/auth/check", post(check_session))

        // Share authentication backend state (PgPool wrapped in Backend)
        .with_state(backend)
}
